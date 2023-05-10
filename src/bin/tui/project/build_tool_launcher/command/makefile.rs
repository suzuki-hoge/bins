use crate::command::item::CommandItem;
use anyhow::Context;
use regex::Regex;
use std::env::current_dir;
use std::fs;
use std::path::Path;

pub fn parse_makefile() -> Vec<CommandItem> {
    let work_dir = current_dir().unwrap();

    _parse_makefile(work_dir)
}

fn _parse_makefile<P: AsRef<Path>>(work_dir: P) -> Vec<CommandItem> {
    match read_makefile(work_dir) {
        Ok(lines) => create_command_items(lines),
        Err(_) => vec![],
    }
}

fn read_makefile<P: AsRef<Path>>(work_dir: P) -> anyhow::Result<String> {
    fs::read_to_string(work_dir.as_ref().join("Makefile")).context("no Makefile")
}

fn create_command_items(origin_lines: String) -> Vec<CommandItem> {
    let label_regex = Regex::new(r"^[^\t]+.*").unwrap();
    let lines_regex = Regex::new(r"^\t").unwrap();

    let mut command_items = vec![];

    let mut label = String::new();
    let mut lines = vec![];

    for line in origin_lines.split('\n') {
        match (label_regex.is_match(line), lines_regex.is_match(line)) {
            (true, false) => {
                if !lines.is_empty() {
                    command_items.push(CommandItem::new(label, lines, false));
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
    command_items.push(CommandItem::new(label, lines, false));

    command_items
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;

    use std::io::Write;
    use std::path::Path;

    use crate::command::item::CommandItem;
    use crate::command::makefile::_parse_makefile;
    use trim_margin::MarginTrimmable;

    fn setup<P: AsRef<Path>>(work_dir: P) {
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

        let work_dir = work_dir.as_ref();
        let _ = fs::create_dir_all(work_dir);
        let _ = File::create(work_dir.join("Makefile")).unwrap().write_all(raw.as_bytes());
    }

    fn cleanup(work_dir: &str) {
        let _ = fs::remove_dir_all(work_dir);
    }

    #[test]
    fn found() {
        let work_dir = "target/build-tool-launcher/test-pj/makefile-found";

        setup(work_dir);

        let act = _parse_makefile(work_dir);
        let commands = vec![
            CommandItem::new("make up", vec!["container up -d"], false),
            CommandItem::new("make down", vec!["container down"], false),
            CommandItem::new("make test", vec!["clear cache", "run test"], false),
        ];

        assert_eq!(act, commands);

        cleanup(work_dir);
    }

    #[test]
    fn notfound() {
        let work_dir = "target/build-tool-launcher/test-pj/makefile-notfound";

        let act = _parse_makefile(work_dir);

        assert_eq!(act, vec![]);
    }
}
