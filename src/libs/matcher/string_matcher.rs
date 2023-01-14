use crate::libs::item::previewable_item::PreviewableItem;
use itertools::Itertools;

type Start = usize;
type End = usize;
type Highlight = bool;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct CheckedString<Item>
where
    Item: PreviewableItem,
{
    origin_item: Item,
    origin_string: String,
    lower_origin: String,
    origin_len: usize,
    matched: bool,
    ranges: Vec<(Start, End, Highlight)>,
}

impl<Item> CheckedString<Item>
where
    Item: PreviewableItem,
{
    pub fn init(origin_item: Item) -> CheckedString<Item> {
        let origin_string = origin_item.get_origin();
        let lower_origin = origin_string.to_lowercase();
        let origin_len = origin_string.len();
        CheckedString {
            origin_item,
            origin_string,
            lower_origin,
            origin_len,
            matched: true,
            ranges: vec![(0, origin_len, false)],
        }
    }

    pub fn get_origin_string(&self) -> String {
        self.origin_string.clone()
    }

    pub fn is_matched(&self) -> bool {
        self.matched
    }

    pub fn get_string_parts(&self, max_width: usize) -> Vec<(String, bool)> {
        let mut string_parts = self
            .ranges
            .iter()
            .map(|&(start, end, highlight)| (self.origin_string[start..end].to_string(), highlight))
            .collect_vec();

        if self.origin_len < max_width {
            string_parts.push((" ".repeat(max_width - self.origin_len), false));
        }

        string_parts
    }

    pub fn show(mut self) -> Self {
        self.matched = true;
        self
    }

    pub fn re_match(mut self, lower_words: &[String]) -> Self {
        self.matched = false;
        self.ranges = vec![(0, self.origin_len, false)];

        for word in lower_words {
            // guard
            if !self.lower_origin.contains(word) {
                return self;
            }

            // check for any hit
            let mut found_once = false;

            self.ranges = self
                .ranges
                .iter()
                .flat_map(|&(s1, e1, highlight)| {
                    if !highlight {
                        if let Some(i) = self.lower_origin[s1..e1].find(word) {
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
                self.matched = false;
                self.ranges = vec![(0, self.origin_len, false)];
                return self;
            }
        }

        self.matched = true;
        self
    }
}

unsafe impl<Item> Send for CheckedString<Item> where Item: PreviewableItem {}

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
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 22, false)]);

        let sut = sut.re_match(&words(&["x"]));
        assert!(!sut.matched);
        assert_eq!(sut.ranges, vec![(0, 22, false)]);

        let sut = sut.re_match(&[]);
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 22, false)]);

        let sut = sut.re_match(&words(&["main"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 13, false), (13, 17, true), (17, 22, false)]);

        let sut = sut.re_match(&words(&["main", "app"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 9, false), (9, 12, true), (12, 13, false), (13, 17, true), (17, 22, false)]);

        let sut = sut.re_match(&words(&["main", "in"]));
        assert!(!sut.matched);
        assert_eq!(sut.ranges, vec![(0, 22, false)]);
    }

    // matched

    #[test]
    fn left_edge_char_matched() {
        let sut = init("abcde");
        let sut = sut.re_match(&words(&["a"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 1, true), (1, 5, false)]);
    }

    #[test]
    fn right_edge_char_matched() {
        let sut = init("abcde");
        let sut = sut.re_match(&words(&["e"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 4, false), (4, 5, true)]);
    }

    #[test]
    fn inside_char_matched() {
        let sut = init("abcde");
        let sut = sut.re_match(&words(&["c"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 2, false), (2, 3, true), (3, 5, false)]);
    }

    #[test]
    fn left_edge_word_matched() {
        let sut = init("abcde");
        let sut = sut.re_match(&words(&["ab"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 2, true), (2, 5, false)]);
    }

    #[test]
    fn right_edge_word_matched() {
        let sut = init("abcde");
        let sut = sut.re_match(&words(&["de"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 3, false), (3, 5, true)]);
    }

    #[test]
    fn inside_word_matched() {
        let sut = init("abcde");
        let sut = sut.re_match(&words(&["bcd"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 1, false), (1, 4, true), (4, 5, false)]);
    }

    #[test]
    fn chars_matched() {
        let sut = init("abcde");
        let sut = sut.re_match(&words(&["a", "c", "e"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 1, true), (1, 2, false), (2, 3, true), (3, 4, false), (4, 5, true)]);
    }

    #[test]
    fn all_chars_matched() {
        let sut = init("abcde");
        let sut = sut.re_match(&words(&["a", "b", "c", "d", "e"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 1, true), (1, 2, true), (2, 3, true), (3, 4, true), (4, 5, true)]);
    }

    #[test]
    fn words_matched() {
        let sut = init("abcde");
        let sut = sut.re_match(&words(&["ab", "de"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 2, true), (2, 3, false), (3, 5, true)]);
    }

    #[test]
    fn rev_words_matched() {
        let sut = init("abcde");
        let sut = sut.re_match(&words(&["de", "ab"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 2, true), (2, 3, false), (3, 5, true)]);
    }

    #[test]
    fn duplicated_words_matched() {
        let sut = init("abccde");
        let sut = sut.re_match(&words(&["abc", "cd"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 3, true), (3, 5, true), (5, 6, false)]);
    }

    #[test]
    fn case_mismatch_words_matched() {
        let sut = init("ABCdeF");
        let sut = sut.re_match(&words(&["ab", "cd", "ef"]));
        assert!(sut.matched);
        assert_eq!(sut.ranges, vec![(0, 2, true), (2, 4, true), (4, 6, true)]);
    }

    // unmatched

    #[test]
    fn not_appear_char_unmatched() {
        let sut = init("abcde");
        let sut = sut.re_match(&words(&["x"]));
        assert!(!sut.matched);
        assert_eq!(sut.ranges, vec![(0, 5, false)]);
    }

    #[test]
    fn not_appear_word_unmatched() {
        let sut = init("abcde");
        let sut = sut.re_match(&words(&["ba"]));
        assert!(!sut.matched);
        assert_eq!(sut.ranges, vec![(0, 5, false)]);
    }

    #[test]
    fn duplicated_words_unmatched() {
        let sut = init("abcde");
        let sut = sut.re_match(&words(&["abc", "cd"]));
        assert!(!sut.matched);
        assert_eq!(sut.ranges, vec![(0, 5, false)]);
    }
}
