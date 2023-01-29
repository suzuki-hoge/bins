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

    pane1_string: String,
    pane1_lower_string: String,
    pane1_string_len: usize,
    pane1_matched: bool,
    pane1_matched_ranges: Vec<(Start, End, Highlight)>,

    pane2_strings: Vec<String>,
    pane2_lower_strings: Vec<String>,
    pane2_matched: bool,
    pane2_matched_ranges_vec: Vec<Vec<(Start, End, Highlight)>>,
}

impl<Item> CheckedString<Item>
where
    Item: DisplayItem,
{
    pub fn init(origin_item: Item) -> CheckedString<Item> {
        let pane1_string = origin_item.get_pane1();
        let pane1_lower_string = pane1_string.to_lowercase();
        let pane1_string_len = pane1_string.len();

        let pane2_strings = origin_item.get_pane2();
        let pane2_lower_strings = pane2_strings.iter().map(|s| s.to_lowercase()).collect_vec();
        CheckedString {
            origin_item,
            pane1_string,
            pane1_lower_string,
            pane1_string_len,
            pane1_matched: true,
            pane1_matched_ranges: vec![(0, pane1_string_len, false)],
            pane2_strings,
            pane2_lower_strings,
            pane2_matched: true,
            pane2_matched_ranges_vec: vec![],
        }
    }

    pub fn get_origin_item(&self) -> Item {
        self.origin_item.clone()
    }

    pub fn is_matched(&self, match_mode: MatchMode) -> bool {
        match match_mode {
            MatchMode::PANE1 => self.pane1_matched,
            MatchMode::PANE2 => self.pane2_matched,
            MatchMode::BOTH => self.pane1_matched || self.pane2_matched,
        }
    }

    pub fn get_pane1_string_parts(&self, max_width: usize) -> Vec<(String, bool)> {
        let mut string_parts = self
            .pane1_matched_ranges
            .iter()
            .map(|&(start, end, highlight)| (self.pane1_string[start..end].to_string(), highlight))
            .collect_vec();

        if self.pane1_string_len < max_width {
            string_parts.push((" ".repeat(max_width - self.pane1_string_len), false));
        }

        string_parts
    }

    pub fn get_pane2_string_parts_vec(&self) -> Vec<Vec<(String, bool)>> {
        self.pane2_matched_ranges_vec
            .iter()
            .enumerate()
            .map(|(i, ranges)| {
                ranges
                    .iter()
                    .map(|&(start, end, highlight)| (self.pane2_strings[i][start..end].to_string(), highlight))
                    .collect_vec()
            })
            .collect_vec()
    }

    pub fn re_match(self, lower_words: &[String], match_mode: MatchMode) -> Self {
        match match_mode {
            MatchMode::PANE1 => self.re_pane1_match(lower_words),
            MatchMode::PANE2 => self.re_pane2_match(lower_words),
            MatchMode::BOTH => self.re_pane1_match(lower_words).re_pane2_match(lower_words),
        }
    }

    fn re_pane1_match(mut self, lower_words: &[String]) -> Self {
        self.pane1_matched = false;
        self.pane1_matched_ranges = vec![(0, self.pane1_string_len, false)];

        for word in lower_words {
            // guard
            if !self.pane1_lower_string.contains(word) {
                return self;
            }

            // check for any hit
            let mut found_once = false;

            self.pane1_matched_ranges = self
                .pane1_matched_ranges
                .iter()
                .flat_map(|&(s1, e1, highlight)| {
                    if !highlight {
                        if let Some(i) = self.pane1_lower_string[s1..e1].find(word) {
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
                self.pane1_matched = false;
                self.pane1_matched_ranges = vec![(0, self.pane1_string_len, false)];
                return self;
            }
        }

        self.pane1_matched = true;
        self
    }

    fn re_pane2_match(mut self, lower_words: &[String]) -> Self {
        self.pane2_matched = false;
        self.pane2_matched_ranges_vec = self.pane2_strings.iter().map(|s| vec![(0, s.len(), false)]).collect_vec();

        for word in lower_words {
            // check for any hit
            let mut found_once = false;

            self.pane2_matched_ranges_vec = self
                .pane2_matched_ranges_vec
                .iter()
                .enumerate()
                .map(|(i, ranges)| {
                    ranges
                        .iter()
                        .flat_map(|&(s1, e1, highlight)| {
                            if !highlight {
                                if let Some(i) = self.pane2_lower_strings[i][s1..e1].find(word) {
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
                self.pane2_matched = false;
                self.pane2_matched_ranges_vec =
                    self.pane2_strings.iter().map(|s| vec![(0, s.len(), false)]).collect_vec();
                return self;
            }
        }

        self.pane2_matched = true;
        self
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum MatchMode {
    PANE1,
    PANE2,
    BOTH,
}

#[cfg(test)]
mod tests {
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
        assert!(sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 22, false)]);

        let sut = sut.re_pane1_match(&words(&["x"]));
        assert!(!sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 22, false)]);

        let sut = sut.re_pane1_match(&[]);
        assert!(sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 22, false)]);

        let sut = sut.re_pane1_match(&words(&["main"]));
        assert!(sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 13, false), (13, 17, true), (17, 22, false)]);

        let sut = sut.re_pane1_match(&words(&["main", "app"]));
        assert!(sut.pane1_matched);
        assert_eq!(
            sut.pane1_matched_ranges,
            vec![(0, 9, false), (9, 12, true), (12, 13, false), (13, 17, true), (17, 22, false)]
        );

        let sut = sut.re_pane1_match(&words(&["main", "in"]));
        assert!(!sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 22, false)]);
    }

    // matched

    #[test]
    fn left_edge_char_matched() {
        let sut = init("abcde");
        let sut = sut.re_pane1_match(&words(&["a"]));
        assert!(sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 1, true), (1, 5, false)]);
    }

    #[test]
    fn right_edge_char_matched() {
        let sut = init("abcde");
        let sut = sut.re_pane1_match(&words(&["e"]));
        assert!(sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 4, false), (4, 5, true)]);
    }

    #[test]
    fn inside_char_matched() {
        let sut = init("abcde");
        let sut = sut.re_pane1_match(&words(&["c"]));
        assert!(sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 2, false), (2, 3, true), (3, 5, false)]);
    }

    #[test]
    fn left_edge_word_matched() {
        let sut = init("abcde");
        let sut = sut.re_pane1_match(&words(&["ab"]));
        assert!(sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 2, true), (2, 5, false)]);
    }

    #[test]
    fn right_edge_word_matched() {
        let sut = init("abcde");
        let sut = sut.re_pane1_match(&words(&["de"]));
        assert!(sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 3, false), (3, 5, true)]);
    }

    #[test]
    fn inside_word_matched() {
        let sut = init("abcde");
        let sut = sut.re_pane1_match(&words(&["bcd"]));
        assert!(sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 1, false), (1, 4, true), (4, 5, false)]);
    }

    #[test]
    fn chars_matched() {
        let sut = init("abcde");
        let sut = sut.re_pane1_match(&words(&["a", "c", "e"]));
        assert!(sut.pane1_matched);
        assert_eq!(
            sut.pane1_matched_ranges,
            vec![(0, 1, true), (1, 2, false), (2, 3, true), (3, 4, false), (4, 5, true)]
        );
    }

    #[test]
    fn all_chars_matched() {
        let sut = init("abcde");
        let sut = sut.re_pane1_match(&words(&["a", "b", "c", "d", "e"]));
        assert!(sut.pane1_matched);
        assert_eq!(
            sut.pane1_matched_ranges,
            vec![(0, 1, true), (1, 2, true), (2, 3, true), (3, 4, true), (4, 5, true)]
        );
    }

    #[test]
    fn words_matched() {
        let sut = init("abcde");
        let sut = sut.re_pane1_match(&words(&["ab", "de"]));
        assert!(sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 2, true), (2, 3, false), (3, 5, true)]);
    }

    #[test]
    fn rev_words_matched() {
        let sut = init("abcde");
        let sut = sut.re_pane1_match(&words(&["de", "ab"]));
        assert!(sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 2, true), (2, 3, false), (3, 5, true)]);
    }

    #[test]
    fn duplicated_words_matched() {
        let sut = init("abccde");
        let sut = sut.re_pane1_match(&words(&["abc", "cd"]));
        assert!(sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 3, true), (3, 5, true), (5, 6, false)]);
    }

    #[test]
    fn case_mismatch_words_matched() {
        let sut = init("ABCdeF");
        let sut = sut.re_pane1_match(&words(&["ab", "cd", "ef"]));
        assert!(sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 2, true), (2, 4, true), (4, 6, true)]);
    }

    // unmatched

    #[test]
    fn not_appear_char_unmatched() {
        let sut = init("abcde");
        let sut = sut.re_pane1_match(&words(&["x"]));
        assert!(!sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 5, false)]);
    }

    #[test]
    fn not_appear_word_unmatched() {
        let sut = init("abcde");
        let sut = sut.re_pane1_match(&words(&["ba"]));
        assert!(!sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 5, false)]);
    }

    #[test]
    fn duplicated_words_unmatched() {
        let sut = init("abcde");
        let sut = sut.re_pane1_match(&words(&["abc", "cd"]));
        assert!(!sut.pane1_matched);
        assert_eq!(sut.pane1_matched_ranges, vec![(0, 5, false)]);
    }
}
