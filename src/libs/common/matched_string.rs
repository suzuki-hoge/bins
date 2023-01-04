#[derive(Debug, Eq, PartialEq, Clone)]
pub struct MatchedString {
    pub origin: String,
    pub parts: Vec<Part>,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Part {
    pub value: String,
    pub matched: bool,
}

impl MatchedString {
    pub fn matched_only(word: &str, origin: &str) -> Option<Self> {
        let word = word.replace(' ', "");
        let mut line = origin.to_string();
        let mut parts: Vec<Part> = vec![];
        let case_sensitive = word.chars().any(|c| c != c.to_ascii_lowercase());

        for char in word.chars() {
            if let Some((lhs, matched, rhs)) = MatchedString::split(char, line, case_sensitive) {
                if !lhs.is_empty() {
                    parts.push(Part { value: lhs.to_string(), matched: false });
                }
                parts.push(Part { value: matched.to_string(), matched: true });
                line = rhs;
            } else {
                return None;
            }
        }

        if !line.is_empty() {
            parts.push(Part { value: line, matched: false });
        }

        Some(Self { origin: origin.to_string(), parts })
    }

    fn split(c: char, s: String, case_sensitive: bool) -> Option<(String, String, String)> {
        let sensitive = |c1: char, c2: char| c1 == c2;
        let insensitive = |c1: char, c2: char| c1 == c2.to_ascii_uppercase() || c1 == c2.to_ascii_lowercase();
        for i in 0..s.len() {
            let c_in_s = s.chars().nth(i).unwrap();
            if (case_sensitive && sensitive(c, c_in_s)) || (!case_sensitive && insensitive(c, c_in_s)) {
                return Some((s[..i].to_string(), c_in_s.to_string(), s[i + 1..].to_string()));
            }
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::common::matched_string::{MatchedString, Part};
    use itertools::Itertools;

    fn exp(tups: Vec<(&str, bool)>) -> Option<MatchedString> {
        let origin = tups.iter().map(|&(s, _)| s.to_string()).join("");
        let parts = tups.iter().map(|tup| Part { value: tup.0.to_string(), matched: tup.1 }).collect();
        Some(MatchedString { origin, parts })
    }

    #[test]
    fn test_ok_edge() {
        let act = MatchedString::matched_only("ae", "abcde");
        let exp = exp(vec![("a", true), ("bcd", false), ("e", true)]);

        assert_eq!(act, exp);
    }

    #[test]
    fn test_ok_not_edge() {
        let act = MatchedString::matched_only("bd", "abcde");
        let exp = exp(vec![("a", false), ("b", true), ("c", false), ("d", true), ("e", false)]);

        assert_eq!(act, exp);
    }

    #[test]
    fn test_ok_inside() {
        let act = MatchedString::matched_only("bcd", "abcde");
        let exp = exp(vec![("a", false), ("b", true), ("c", true), ("d", true), ("e", false)]);

        assert_eq!(act, exp);
    }

    #[test]
    fn test_ok_case_insensitive() {
        let act = MatchedString::matched_only("abc", "ABCDE");
        let exp = exp(vec![("A", true), ("B", true), ("C", true), ("DE", false)]);

        assert_eq!(act, exp);
    }

    #[test]
    fn test_ok_case_sensitive() {
        let act = MatchedString::matched_only("Abc", "Abcde");
        let exp = exp(vec![("A", true), ("b", true), ("c", true), ("de", false)]);

        assert_eq!(act, exp);
    }

    #[test]
    fn test_ok_empty() {
        let act = MatchedString::matched_only("", "abcde");
        let exp = exp(vec![("abcde", false)]);

        assert_eq!(act, exp);
    }

    #[test]
    fn test_ok_space_only() {
        let act = MatchedString::matched_only(" ", "abcde");
        let exp = exp(vec![("abcde", false)]);

        assert_eq!(act, exp);
    }

    #[test]
    fn test_ok_include_space() {
        let act = MatchedString::matched_only("a b", "abcde");
        let exp = exp(vec![("a", true), ("b", true), ("cde", false)]);

        assert_eq!(act, exp);
    }

    #[test]
    fn test_ng_order_mismatch() {
        let act = MatchedString::matched_only("ba", "abcde");

        assert_eq!(act, None);
    }

    #[test]
    fn test_ng_not_appeared() {
        let act = MatchedString::matched_only("ef", "abcde");

        assert_eq!(act, None);
    }

    #[test]
    fn test_ng_case_sensitive() {
        let act = MatchedString::matched_only("abC", "Abcde");

        assert_eq!(act, None);
    }
}
