use crate::command::parsed_command::{ParsedCommand, ParsedContent};
use regex::Regex;
use std::fs;

pub fn parse_makefile(path: String) -> ParsedCommand {
    match fs::read_to_string(path) {
        Ok(lines) => parse(lines),
        Err(_) => ParsedCommand::empty(),
    }
}

fn parse(lines: String) -> ParsedCommand {
    let key_r = Regex::new(r"^[^\t]+.*").unwrap();
    let body_r = Regex::new(r"^\t").unwrap();

    let mut contents = vec![];

    let mut key = String::new();
    let mut bodies = vec![];

    for line in lines.split('\n') {
        match (key_r.is_match(line), body_r.is_match(line)) {
            (true, false) => {
                if !bodies.is_empty() {
                    contents.push(ParsedContent::new(key, bodies));
                    bodies = vec![]
                }
                key = format!("make {}", line.replace(':', ""))
            }
            (false, true) => bodies.push(line.trim().to_string()),
            (_, _) => {
                // do nothing
            }
        };
    }
    contents.push(ParsedContent::new(key, bodies));

    ParsedCommand::new(contents)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;

    use std::io::Write;

    use crate::command::parsed_command::ParsedCommand;
    use crate::{parse_makefile, ParsedContent};
    use trim_margin::MarginTrimmable;

    fn setup(path: &str) {
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

        File::create(path).unwrap().write_all(raw.as_bytes()).unwrap();
    }

    fn cleanup(path: &str) {
        let _ = fs::remove_file(path);
    }

    #[test]
    fn found() {
        let path = "/tmp/Makefile";

        setup(path);

        let sut = parse_makefile(path.to_string());
        let contents = vec![
            ParsedContent::new("make up".to_string(), vec!["container up -d".to_string()]),
            ParsedContent::new("make down".to_string(), vec!["container down".to_string()]),
            ParsedContent::new("make test".to_string(), vec!["clear cache".to_string(), "run test".to_string()]),
        ];

        assert_eq!(sut, ParsedCommand::new(contents));

        cleanup(path);
    }

    #[test]
    fn notfound() {
        let path = "/tmp/Makefile";

        cleanup(path);

        let sut = parse_makefile(path.to_string());

        assert_eq!(sut, ParsedCommand::empty());
    }
}
