#[derive(Debug)]
pub struct Matcher {
    words: Vec<String>,
}

impl Matcher {
    pub fn new(input: &str) -> Self {
        Self { words: input.to_ascii_lowercase().split(' ').map(String::from).collect() }
    }

    pub fn is_match(&self, line: &str) -> bool {
        let mut line = line.to_ascii_lowercase();

        for word in &self.words {
            if !line.contains(word) {
                return false;
            }
            line = line.replacen(word, "", 1);
        }
        true
    }

    pub fn get_matched_parts(&self, origin_line: &str) -> Vec<(String, bool)> {
        let line = origin_line.to_ascii_lowercase();

        let mut line_parts = vec![((0, line.len()), false)];

        for word in &self.words {
            line_parts = line_parts
                .iter()
                .flat_map(|&((s, e), b)| {
                    if b {
                        vec![((s, e), true)]
                    } else if let Some(i) = line[s..e].find(word) {
                        vec![((s, s + i), false), ((s + i, s + i + word.len()), true), ((s + i + word.len(), e), false)]
                    } else {
                        vec![((s, e), false)]
                    }
                })
                .collect();
        }

        line_parts
            .into_iter()
            .filter(|((s, e), _)| s != e)
            .map(|((s, e), b)| (origin_line[s..e].to_string(), b))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::fuzzy::matcher::Matcher;

    #[test]
    fn is_match() {
        let line = "/FOO/Bar/app/main.rs";

        assert!(Matcher::new("").is_match(line));

        assert!(Matcher::new("").is_match(line));
        assert!(Matcher::new(" ").is_match(line));

        assert!(Matcher::new("foo").is_match(line));
        assert!(Matcher::new("bar").is_match(line));
        assert!(Matcher::new("/app").is_match(line));
        assert!(Matcher::new("main.rs").is_match(line));

        assert!(Matcher::new("foo bar").is_match(line));
        assert!(Matcher::new("bar foo").is_match(line));

        assert!(Matcher::new("main rs").is_match(line));
        assert!(Matcher::new("rs main").is_match(line));

        assert!(Matcher::new("f o o").is_match(line));
        assert!(Matcher::new("a a a").is_match(line));

        assert!(!Matcher::new("fooo").is_match(line));

        assert!(!Matcher::new("foo lib").is_match(line));

        assert!(!Matcher::new("//").is_match(line));
        assert!(!Matcher::new("foobar").is_match(line));

        assert!(!Matcher::new("foo foo").is_match(line));
        assert!(!Matcher::new("f o o o").is_match(line));
    }

    fn o(s: &str) -> (String, bool) {
        (s.to_string(), true)
    }

    fn x(s: &str) -> (String, bool) {
        (s.to_string(), false)
    }

    #[test]
    fn test_get_matched_parts() {
        let line = "/FOO/Bar/app/main.rs";

        assert_eq!(Matcher::new("app").get_matched_parts(line), vec![x("/FOO/Bar/"), o("app"), x("/main.rs")]);

        assert_eq!(Matcher::new("/").get_matched_parts(line), vec![o("/"), x("FOO/Bar/app/main.rs")]);
        assert_eq!(Matcher::new("/f").get_matched_parts(line), vec![o("/F"), x("OO/Bar/app/main.rs")]);

        assert_eq!(Matcher::new("s").get_matched_parts(line), vec![x("/FOO/Bar/app/main.r"), o("s")]);
        assert_eq!(Matcher::new("rs").get_matched_parts(line), vec![x("/FOO/Bar/app/main."), o("rs")]);

        assert_eq!(Matcher::new("/foo/bar/app/main.rs").get_matched_parts(line), vec![o("/FOO/Bar/app/main.rs")]);

        assert_eq!(
            Matcher::new("foo bar").get_matched_parts(line),
            vec![x("/"), o("FOO"), x("/"), o("Bar"), x("/app/main.rs")]
        );
        assert_eq!(
            Matcher::new("bar foo").get_matched_parts(line),
            vec![x("/"), o("FOO"), x("/"), o("Bar"), x("/app/main.rs")]
        );
        assert_eq!(
            Matcher::new("a a a").get_matched_parts(line),
            vec![x("/FOO/B"), o("a"), x("r/"), o("a"), x("pp/m"), o("a"), x("in.rs"),]
        );

        assert_eq!(Matcher::new("struct").get_matched_parts(line), vec![x("/FOO/Bar/app/main.rs")]);
    }
}
