use crate::command::command_item::CommandItem;
use anyhow::Context;
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn parse_makefile(dir_path: &Path) -> Vec<CommandItem> {
    match read_file(dir_path) {
        Ok(lines) => parse(lines),
        Err(_) => vec![],
    }
}

fn read_file(dir_path: &Path) -> anyhow::Result<String> {
    fs::read_to_string(dir_path.join("Makefile")).context("no Makefile")
}

fn parse(origin_lines: String) -> Vec<CommandItem> {
    let label_regex = Regex::new(r"^[^\t]+.*").unwrap();
    let lines_regex = Regex::new(r"^\t").unwrap();

    let mut commands = vec![];

    let mut label = String::new();
    let mut lines = vec![];

    for line in origin_lines.split('\n') {
        match (label_regex.is_match(line), lines_regex.is_match(line)) {
            (true, false) => {
                if !lines.is_empty() {
                    commands.push(CommandItem::new(label, lines));
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
    commands.push(CommandItem::new(label, lines));

    commands
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;

    use std::io::Write;
    use std::path::PathBuf;

    use crate::command::command_item::CommandItem;
    use crate::parse_makefile;
    use trim_margin::MarginTrimmable;

    fn setup(dir_path: &PathBuf) {
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

        let _ = fs::create_dir_all(dir_path);
        let _ = File::create(dir_path.join("Makefile")).unwrap().write_all(raw.as_bytes());
    }

    fn cleanup(dir_path: &PathBuf) {
        let _ = fs::remove_dir_all(dir_path);
    }

    #[test]
    fn found() {
        let dir_path = PathBuf::from("target/build-tool-launcher/test-pj/makefile-found");

        setup(&dir_path);

        let sut = parse_makefile(&dir_path);
        let commands = vec![
            CommandItem::new("make up".to_string(), vec!["container up -d".to_string()]),
            CommandItem::new("make down".to_string(), vec!["container down".to_string()]),
            CommandItem::new("make test".to_string(), vec!["clear cache".to_string(), "run test".to_string()]),
        ];

        assert_eq!(sut, commands);

        cleanup(&dir_path);
    }

    #[test]
    fn notfound() {
        let dir_path = PathBuf::from("target/build-tool-launcher/test-pj/makefile-notfound");

        let sut = parse_makefile(&dir_path);

        assert_eq!(sut, vec![]);
    }
}
