use crate::command::command_item::CommandItem;
use anyhow::Context;
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn parse_makefile(work_dir: &Path) -> Vec<CommandItem> {
    match read_file(work_dir) {
        Ok(lines) => create_command_items(lines),
        Err(_) => vec![],
    }
}

fn read_file(work_dir: &Path) -> anyhow::Result<String> {
    fs::read_to_string(work_dir.join("Makefile")).context("no Makefile")
}

fn create_command_items(origin_lines: String) -> Vec<CommandItem> {
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

    fn setup(work_dir: &PathBuf) {
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

        let _ = fs::create_dir_all(work_dir);
        let _ = File::create(work_dir.join("Makefile")).unwrap().write_all(raw.as_bytes());
    }

    fn cleanup(work_dir: &PathBuf) {
        let _ = fs::remove_dir_all(work_dir);
    }

    #[test]
    fn found() {
        let work_dir = PathBuf::from("target/build-tool-launcher/test-pj/makefile-found");

        setup(&work_dir);

        let act = parse_makefile(&work_dir);
        let commands = vec![
            CommandItem::new("make up".to_string(), vec!["container up -d".to_string()]),
            CommandItem::new("make down".to_string(), vec!["container down".to_string()]),
            CommandItem::new("make test".to_string(), vec!["clear cache".to_string(), "run test".to_string()]),
        ];

        assert_eq!(act, commands);

        cleanup(&work_dir);
    }

    #[test]
    fn notfound() {
        let work_dir = PathBuf::from("target/build-tool-launcher/test-pj/makefile-notfound");

        let act = parse_makefile(&work_dir);

        assert_eq!(act, vec![]);
    }
}
