use crate::item::parser::{CommandType, ParsedItem, Parser};
use regex::Regex;
use std::fs;
use std::path::Path;

pub struct MakefileParser;

impl Parser for MakefileParser {
    fn get_items(&self, dir: &Path) -> Vec<ParsedItem> {
        let path = dir.join("Makefile");
        if path.exists() {
            let label_regex = Regex::new(r"^[^\t]+.*").unwrap();
            let lines_regex = Regex::new(r"^\t").unwrap();

            let mut parsed_items = vec![];

            let mut label = String::new();
            let mut lines = vec![];

            for line in fs::read_to_string(path).unwrap().split("\n") {
                match (label_regex.is_match(line), lines_regex.is_match(line)) {
                    (true, false) => {
                        if !lines.is_empty() {
                            parsed_items.push(ParsedItem {
                                label,
                                lines,
                                args: vec![],
                                command_type: CommandType::Pass,
                            });
                            lines = vec![]
                        }
                        label = format!("make {}", line.replace(':', ""))
                    }
                    (false, true) => lines.push(line.trim().to_string()),
                    (_, _) => {
                        // do nothing
                    }
                };
            }
            parsed_items.push(ParsedItem { label, lines, args: vec![], command_type: CommandType::Pass });

            parsed_items
        } else {
            vec![]
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;

    use std::io::Write;
    use std::path::{Path, PathBuf};

    use crate::item::makefile_parser::MakefileParser;
    use crate::item::parser::{CommandType, ParsedItem, Parser};
    use trim_margin::MarginTrimmable;

    fn setup<P: AsRef<Path>>(dir: P) {
        let raw = "
            |up:
            |	container up -d
            |down:
            |	container down
            |
            |test:
            |	clear cache
            |	run test
        "
        .trim()
        .trim_margin()
        .unwrap();

        let dir = dir.as_ref();
        let _ = fs::create_dir_all(dir);
        let _ = File::create(dir.join("Makefile")).unwrap().write_all(raw.as_bytes());
    }

    fn cleanup<P: AsRef<Path>>(dir: P) {
        let _ = fs::remove_dir_all(dir);
    }

    fn s(s: &str) -> String {
        s.to_string()
    }

    #[test]
    fn found() {
        let dir = PathBuf::from("target/project/task/test-project/makefile/found");

        setup(&dir);

        let act = MakefileParser.get_items(&dir);
        let items = vec![
            ParsedItem {
                label: s("make up"),
                lines: vec![s("container up -d")],
                args: vec![],
                command_type: CommandType::Pass,
            },
            ParsedItem {
                label: s("make down"),
                lines: vec![s("container down")],
                args: vec![],
                command_type: CommandType::Pass,
            },
            ParsedItem {
                label: s("make test"),
                lines: vec![s("clear cache"), s("run test")],
                args: vec![],
                command_type: CommandType::Pass,
            },
        ];

        assert_eq!(act, items);

        cleanup(&dir);
    }

    #[test]
    fn notfound() {
        let dir = PathBuf::from("target/project/task/test-project/makefile/notfound");

        let act = MakefileParser.get_items(&dir);

        assert_eq!(act, vec![]);
    }
}
