use crate::libs::item::display_item::DisplayItem;
use itertools::Itertools;

type Start = usize;
type End = usize;
type Highlight = bool;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct CheckedString<Item>
where
    Item: DisplayItem,
{
    origin_item: Item,

    origin_string: String,
    origin_lower_string: String,
    origin_string_len: usize,
    origin_matched: bool,
    origin_matched_ranges: Vec<(Start, End, Highlight)>,

    preview_strings: Vec<String>,
    preview_lower_strings: Vec<String>,
    preview_matched: bool,
    preview_matched_ranges_vec: Vec<Vec<(Start, End, Highlight)>>,
}

impl<Item> CheckedString<Item>
where
    Item: DisplayItem,
{
    pub fn init(origin_item: Item) -> CheckedString<Item> {
        let origin_string = origin_item.get_pane1();
        let lower_origin = origin_string.to_lowercase();
        let origin_len = origin_string.len();

        let preview_strings = origin_item.get_pane2();
        let preview_lower_strings = preview_strings.iter().map(|s| s.to_lowercase()).collect_vec();
        CheckedString {
            origin_item,
            origin_string,
            origin_lower_string: lower_origin,
            origin_string_len: origin_len,
            origin_matched: true,
            origin_matched_ranges: vec![(0, origin_len, false)],
            preview_strings,
            preview_lower_strings,
            preview_matched: true,
            preview_matched_ranges_vec: vec![],
        }
    }

    pub fn get_origin_item(&self) -> Item {
        self.origin_item.clone()
    }

    pub fn is_matched(&self, mode: Mode) -> bool {
        match mode {
            Mode::ORIGIN => self.origin_matched,
            Mode::PREVIEW => self.preview_matched,
            Mode::BOTH => self.origin_matched || self.preview_matched,
        }
    }

    pub fn get_origin_string_parts(&self, max_width: usize) -> Vec<(String, bool)> {
        let mut string_parts = self
            .origin_matched_ranges
            .iter()
            .map(|&(start, end, highlight)| (self.origin_string[start..end].to_string(), highlight))
            .collect_vec();

        if self.origin_string_len < max_width {
            string_parts.push((" ".repeat(max_width - self.origin_string_len), false));
        }

        string_parts
    }

    pub fn get_preview_string_parts_vec(&self) -> Vec<Vec<(String, bool)>> {
        self.preview_matched_ranges_vec
            .iter()
            .enumerate()
            .map(|(i, ranges)| {
                ranges
                    .iter()
                    .map(|&(start, end, highlight)| (self.preview_strings[i][start..end].to_string(), highlight))
                    .collect_vec()
            })
            .collect_vec()
    }

    pub fn re_match(self, lower_words: &[String], mode: Mode) -> Self {
        match mode {
            Mode::ORIGIN => self.re_origin_match(lower_words),
            Mode::PREVIEW => self.re_preview_match(lower_words),
            Mode::BOTH => self.re_origin_match(lower_words).re_preview_match(lower_words),
        }
    }

    fn re_origin_match(mut self, lower_words: &[String]) -> Self {
        self.origin_matched = false;
        self.origin_matched_ranges = vec![(0, self.origin_string_len, false)];

        for word in lower_words {
            // guard
            if !self.origin_lower_string.contains(word) {
                return self;
            }

            // check for any hit
            let mut found_once = false;

            self.origin_matched_ranges = self
                .origin_matched_ranges
                .iter()
                .flat_map(|&(s1, e1, highlight)| {
                    if !highlight {
                        if let Some(i) = self.origin_lower_string[s1..e1].find(word) {
                            found_once = true;

                            let (e2, e3) = (s1 + i, s1 + i + word.len());

                            if s1 != e2 && e3 != e1 {
                                vec![(s1, e2, false), (e2, e3, true), (e3, e1, false)]
                            } else if s1 != e2 {
                                vec![(s1, e2, false), (e2, e3, true)]
                            } else if e3 != e1 {
                                vec![(e2, e3, true), (e3, e1, false)]
                            } else {
                                vec![(e2, e3, true)]
                            }
                        } else {
                            vec![(s1, e1, highlight)]
                        }
                    } else {
                        vec![(s1, e1, highlight)]
                    }
                })
                .collect_vec();

            if !found_once {
                self.origin_matched = false;
                self.origin_matched_ranges = vec![(0, self.origin_string_len, false)];
                return self;
            }
        }

        self.origin_matched = true;
        self
    }

    fn re_preview_match(mut self, lower_words: &[String]) -> Self {
        self.preview_matched = false;
        self.preview_matched_ranges_vec = self.preview_strings.iter().map(|s| vec![(0, s.len(), false)]).collect_vec();

        for word in lower_words {
            // check for any hit
            let mut found_once = false;

            self.preview_matched_ranges_vec = self
                .preview_matched_ranges_vec
                .iter()
                .enumerate()
                .map(|(i, ranges)| {
                    ranges
                        .iter()
                        .flat_map(|&(s1, e1, highlight)| {
                            if !highlight {
                                if let Some(i) = self.preview_lower_strings[i][s1..e1].find(word) {
                                    found_once = true;

                                    let (e2, e3) = (s1 + i, s1 + i + word.len());

                                    if s1 != e2 && e3 != e1 {
                                        vec![(s1, e2, false), (e2, e3, true), (e3, e1, false)]
                                    } else if s1 != e2 {
                                        vec![(s1, e2, false), (e2, e3, true)]
                                    } else if e3 != e1 {
                                        vec![(e2, e3, true), (e3, e1, false)]
                                    } else {
                                        vec![(e2, e3, true)]
                                    }
                                } else {
                                    vec![(s1, e1, highlight)]
                                }
                            } else {
                                vec![(s1, e1, highlight)]
                            }
                        })
                        .collect_vec()
                })
                .collect_vec();

            if !found_once {
                self.preview_matched = false;
                self.preview_matched_ranges_vec =
                    self.preview_strings.iter().map(|s| vec![(0, s.len(), false)]).collect_vec();
                return self;
            }
        }

        self.preview_matched = true;
        self
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum Mode {
    ORIGIN,
    PREVIEW,
    BOTH,
}

#[cfg(test)]
mod origin_tests {
    use crate::libs::matcher::string_matcher::CheckedString;
    use itertools::Itertools;

    fn init(s: &str) -> CheckedString<String> {
        CheckedString::init(s.to_string())
    }

    fn words(ss: &[&str]) -> Vec<String> {
        ss.iter().map(|s| s.to_string()).collect_vec()
    }

    // flow

    #[test]
    fn flow() {
        let sut = init("/foo/bar/app/Main.java");
        assert!(sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 22, false)]);

        let sut = sut.re_origin_match(&words(&["x"]));
        assert!(!sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 22, false)]);

        let sut = sut.re_origin_match(&[]);
        assert!(sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 22, false)]);

        let sut = sut.re_origin_match(&words(&["main"]));
        assert!(sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 13, false), (13, 17, true), (17, 22, false)]);

        let sut = sut.re_origin_match(&words(&["main", "app"]));
        assert!(sut.origin_matched);
        assert_eq!(
            sut.origin_matched_ranges,
            vec![(0, 9, false), (9, 12, true), (12, 13, false), (13, 17, true), (17, 22, false)]
        );

        let sut = sut.re_origin_match(&words(&["main", "in"]));
        assert!(!sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 22, false)]);
    }

    // matched

    #[test]
    fn left_edge_char_matched() {
        let sut = init("abcde");
        let sut = sut.re_origin_match(&words(&["a"]));
        assert!(sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 1, true), (1, 5, false)]);
    }

    #[test]
    fn right_edge_char_matched() {
        let sut = init("abcde");
        let sut = sut.re_origin_match(&words(&["e"]));
        assert!(sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 4, false), (4, 5, true)]);
    }

    #[test]
    fn inside_char_matched() {
        let sut = init("abcde");
        let sut = sut.re_origin_match(&words(&["c"]));
        assert!(sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 2, false), (2, 3, true), (3, 5, false)]);
    }

    #[test]
    fn left_edge_word_matched() {
        let sut = init("abcde");
        let sut = sut.re_origin_match(&words(&["ab"]));
        assert!(sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 2, true), (2, 5, false)]);
    }

    #[test]
    fn right_edge_word_matched() {
        let sut = init("abcde");
        let sut = sut.re_origin_match(&words(&["de"]));
        assert!(sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 3, false), (3, 5, true)]);
    }

    #[test]
    fn inside_word_matched() {
        let sut = init("abcde");
        let sut = sut.re_origin_match(&words(&["bcd"]));
        assert!(sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 1, false), (1, 4, true), (4, 5, false)]);
    }

    #[test]
    fn chars_matched() {
        let sut = init("abcde");
        let sut = sut.re_origin_match(&words(&["a", "c", "e"]));
        assert!(sut.origin_matched);
        assert_eq!(
            sut.origin_matched_ranges,
            vec![(0, 1, true), (1, 2, false), (2, 3, true), (3, 4, false), (4, 5, true)]
        );
    }

    #[test]
    fn all_chars_matched() {
        let sut = init("abcde");
        let sut = sut.re_origin_match(&words(&["a", "b", "c", "d", "e"]));
        assert!(sut.origin_matched);
        assert_eq!(
            sut.origin_matched_ranges,
            vec![(0, 1, true), (1, 2, true), (2, 3, true), (3, 4, true), (4, 5, true)]
        );
    }

    #[test]
    fn words_matched() {
        let sut = init("abcde");
        let sut = sut.re_origin_match(&words(&["ab", "de"]));
        assert!(sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 2, true), (2, 3, false), (3, 5, true)]);
    }

    #[test]
    fn rev_words_matched() {
        let sut = init("abcde");
        let sut = sut.re_origin_match(&words(&["de", "ab"]));
        assert!(sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 2, true), (2, 3, false), (3, 5, true)]);
    }

    #[test]
    fn duplicated_words_matched() {
        let sut = init("abccde");
        let sut = sut.re_origin_match(&words(&["abc", "cd"]));
        assert!(sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 3, true), (3, 5, true), (5, 6, false)]);
    }

    #[test]
    fn case_mismatch_words_matched() {
        let sut = init("ABCdeF");
        let sut = sut.re_origin_match(&words(&["ab", "cd", "ef"]));
        assert!(sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 2, true), (2, 4, true), (4, 6, true)]);
    }

    // unmatched

    #[test]
    fn not_appear_char_unmatched() {
        let sut = init("abcde");
        let sut = sut.re_origin_match(&words(&["x"]));
        assert!(!sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 5, false)]);
    }

    #[test]
    fn not_appear_word_unmatched() {
        let sut = init("abcde");
        let sut = sut.re_origin_match(&words(&["ba"]));
        assert!(!sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 5, false)]);
    }

    #[test]
    fn duplicated_words_unmatched() {
        let sut = init("abcde");
        let sut = sut.re_origin_match(&words(&["abc", "cd"]));
        assert!(!sut.origin_matched);
        assert_eq!(sut.origin_matched_ranges, vec![(0, 5, false)]);
    }
}
