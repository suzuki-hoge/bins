use crate::item::parser::{Arg, CommandType, ParsedItem, Parser};
use bins::io::file::{get_bins_dir, read_deserializable};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::path::{Path, PathBuf};

pub fn get_config_path() -> PathBuf {
    current_dir().unwrap().join("task.b.yml")
}

pub struct TaskParser;

impl Parser for TaskParser {
    fn get_items(&self, dir: &Path) -> Vec<ParsedItem> {
        parse(dir.join("task.b.yml"))
    }
}

fn parse<P: AsRef<Path>>(path: P) -> Vec<ParsedItem> {
    match read_deserializable::<P, Yaml>(path) {
        Ok(yaml) => {
            let include_items = yaml
                .include
                .iter()
                .flat_map(|include| {
                    let path = get_bins_dir().join("commands").join(format!("{include}.yml"));
                    parse(path)
                })
                .collect_vec();

            let items = yaml.commands.into_iter().map(ParsedItem::from).collect();

            vec![include_items, items].into_iter().flatten().collect()
        }
        Err(_) => vec![],
    }
}

#[derive(Serialize, Deserialize)]
struct Yaml {
    include: Vec<String>,
    commands: Vec<YamlCommand>,
}

#[derive(Serialize, Deserialize)]
struct YamlCommand {
    label: String,
    r#type: String,
    lines: String,
    args: Option<Vec<YamlArg>>,
}

#[derive(Serialize, Deserialize)]
struct YamlArg {
    name: String,
    default: Option<String>,
}

impl From<YamlCommand> for ParsedItem {
    fn from(y: YamlCommand) -> Self {
        let label = format!("bb {}", y.label);
        let lines = y.lines.split("\n").map(|s| s.into()).collect();
        let args = match y.args {
            Some(args) => args.into_iter().map(|a| Arg { name: a.name, default: a.default }).collect(),
            None => vec![],
        };
        let command_type = match y.r#type.as_str() {
            "run" => CommandType::Run,
            "edit" => CommandType::Edit,
            "copy" => CommandType::Copy,
            _ => panic!("invalid command type: {}", y.r#type),
        };
        Self { label, lines, args, command_type }
    }
}

#[cfg(test)]
mod tests {
    use crate::item::parser::{Arg, CommandType, ParsedItem, Parser};
    use crate::item::task_parser::TaskParser;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::{Path, PathBuf};
    use trim_margin::MarginTrimmable;

    fn setup<P: AsRef<Path>>(dir: P) {
        let raw = "
            |include: []
            |commands:
            |  - label: run
            |    type: run
            |    lines: |-
            |      run command 1
            |
            |  - label: run multi
            |    type: run
            |    lines: |-
            |      run command 1
            |      run command 2
            |
            |  - label: run args
            |    type: run
            |    lines: |-
            |      run command 1 $arg1$ $arg2$
            |    args: 
            |      - name: arg1
            |        default: val1
            |      - name: arg2
            |
            |  - label: query
            |    type: edit
            |    lines: |-
            |      begin;
            |      update;
            |      commit;
            |
            |  - label: staging
            |    type: copy
            |    lines: |-
            |      93ngpp239g
        "
        .trim()
        .trim_margin()
        .unwrap();

        let dir = dir.as_ref();
        let _ = fs::create_dir_all(dir);
        let _ = File::create(dir.join("task.b.yml")).unwrap().write_all(raw.as_bytes());
    }

    fn cleanup<P: AsRef<Path>>(dir: P) {
        let _ = fs::remove_dir_all(dir);
    }

    fn s(s: &str) -> String {
        s.to_string()
    }

    #[test]
    fn found() {
        let dir = PathBuf::from("target/project/task/test-project/task/found");

        setup(&dir);

        let act = TaskParser.get_items(&dir);
        let items = vec![
            ParsedItem {
                label: s("bb run"),
                lines: vec![s("run command 1")],
                args: vec![],
                command_type: CommandType::Run,
            },
            ParsedItem {
                label: s("bb run multi"),
                lines: vec![s("run command 1"), s("run command 2")],
                args: vec![],
                command_type: CommandType::Run,
            },
            ParsedItem {
                label: s("bb run args"),
                lines: vec![s("run command 1 $arg1$ $arg2$")],
                args: vec![Arg { name: s("arg1"), default: Some(s("val1")) }, Arg { name: s("arg2"), default: None }],
                command_type: CommandType::Run,
            },
            ParsedItem {
                label: s("bb query"),
                lines: vec![s("begin;"), s("update;"), s("commit;")],
                args: vec![],
                command_type: CommandType::Edit,
            },
            ParsedItem {
                label: s("bb staging"),
                lines: vec![s("93ngpp239g")],
                args: vec![],
                command_type: CommandType::Copy,
            },
        ];

        assert_eq!(act, items);

        cleanup(&dir);
    }

    #[test]
    fn notfound() {
        let dir = PathBuf::from("target/project/task/test-project/task/notfound");

        let act = TaskParser.get_items(&dir);

        assert_eq!(act, vec![]);
    }
}
