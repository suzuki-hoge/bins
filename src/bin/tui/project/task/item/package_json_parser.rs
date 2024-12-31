use crate::item::package_json_parser::Tool::{Npm, Yarn};
use crate::item::parser::{CommandType, ParsedItem, Parser};
use bins::io::file::read_deserializable;
use itertools::Itertools;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

pub struct PackageJsonParser;

impl Parser for PackageJsonParser {
    fn get_items(&self, dir: &Path) -> Vec<ParsedItem> {
        match read_deserializable::<PathBuf, PackageJson>(dir.join("package.json")) {
            Ok(package_json) => {
                let tool = match (dir.join(Npm.lock_file_name()).exists(), dir.join(Yarn.lock_file_name()).exists()) {
                    (_, true) => Yarn,
                    _ => Npm,
                };

                package_json
                    .scripts
                    .iter()
                    .sorted()
                    .map(|(key, val)| ParsedItem {
                        label: format!("{} {}", tool.runner(), key),
                        lines: vec![val.to_string()],
                        args: vec![],
                        command_type: CommandType::Pass,
                    })
                    .collect_vec()
            }
            Err(_) => vec![],
        }
    }
}

#[derive(Deserialize, Debug)]
struct PackageJson {
    scripts: HashMap<String, String>,
}

#[derive(Eq, PartialEq, Debug)]
enum Tool {
    Npm,
    Yarn,
}

impl Tool {
    fn runner(&self) -> &str {
        match self {
            Npm => "npm run",
            Yarn => "yarn",
        }
    }

    fn lock_file_name(&self) -> &str {
        match self {
            Npm => "package-lock.json",
            Yarn => "yarn.lock",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;

    use std::io::Write;
    use std::path::{Path, PathBuf};

    use crate::item::package_json_parser::Tool::{Npm, Yarn};
    use crate::item::package_json_parser::{PackageJsonParser, Tool};
    use crate::item::parser::{CommandType, ParsedItem, Parser};
    use trim_margin::MarginTrimmable;
    use CommandType::Pass;

    fn setup<P: AsRef<Path>>(dir: P, tool: Tool) {
        let raw = r#"
            |{
            |  "name": "foo",
            |  "scripts": {
            |    "dev": "dev && start",
            |    "build": "build",
            |    "fmt": "fmt \"*.{ts}\""
            |  }
            |}
        "#
        .trim()
        .trim_margin()
        .unwrap();

        let dir = dir.as_ref();
        let _ = fs::create_dir_all(dir);
        let _ = File::create(dir.join("package.json")).unwrap().write_all(raw.as_bytes());
        let _ = File::create(dir.join(tool.lock_file_name()));
    }

    fn cleanup<P: AsRef<Path>>(dir: P) {
        let _ = fs::remove_dir_all(dir);
    }

    fn s(s: &str) -> String {
        s.to_string()
    }

    #[test]
    fn found_npm() {
        let dir = PathBuf::from("target/project/task/test-project/package-json/found-npm");

        setup(&dir, Npm);

        let act = PackageJsonParser.get_items(&dir);
        let items = vec![
            ParsedItem { label: s("npm run build"), lines: vec![s("build")], args: vec![], command_type: Pass },
            ParsedItem { label: s("npm run dev"), lines: vec![s("dev && start")], args: vec![], command_type: Pass },
            ParsedItem { label: s("npm run fmt"), lines: vec![s(r#"fmt "*.{ts}""#)], args: vec![], command_type: Pass },
        ];

        assert_eq!(act, items);

        cleanup(&dir);
    }

    #[test]
    fn found_yarn() {
        let dir = PathBuf::from("target/project/task/test-project/package-json/found-yarn");

        setup(&dir, Yarn);

        let act = PackageJsonParser.get_items(&dir);
        let items = vec![
            ParsedItem { label: s("yarn build"), lines: vec![s("build")], args: vec![], command_type: Pass },
            ParsedItem { label: s("yarn dev"), lines: vec![s("dev && start")], args: vec![], command_type: Pass },
            ParsedItem { label: s("yarn fmt"), lines: vec![s(r#"fmt "*.{ts}""#)], args: vec![], command_type: Pass },
        ];

        assert_eq!(act, items);

        cleanup(&dir);
    }

    #[test]
    fn notfound() {
        let dir = PathBuf::from("target/project/task/test-project/package-json/notfound");

        let act = PackageJsonParser.get_items(&dir);

        assert_eq!(act, vec![]);
    }
}
