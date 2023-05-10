use std::collections::HashMap;
use std::env::current_dir;
use std::path::Path;

use bins::io::file::read_deserializable;
use itertools::Itertools;
use serde::Deserialize;

use crate::command::item::CommandItem;
use crate::command::package_json::Tool::{Npm, Yarn};

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
    fn get_runner(&self) -> &str {
        match self {
            Npm => "npm run",
            Yarn => "yarn",
        }
    }

    fn get_lock_file_name(&self) -> &str {
        match self {
            Npm => "package-lock.json",
            Yarn => "yarn.lock",
        }
    }
}

pub fn parse_package_json() -> Vec<CommandItem> {
    let work_dir = current_dir().unwrap();

    _parse_package_json(work_dir)
}

fn _parse_package_json<P: AsRef<Path>>(work_dir: P) -> Vec<CommandItem> {
    match read_deserializable(work_dir.as_ref().join("package.json")) {
        Ok(json) => create_command_items(json, find_tool(work_dir)),
        Err(_) => vec![],
    }
}

fn find_tool<P: AsRef<Path>>(work_dir: P) -> Tool {
    let npm = work_dir.as_ref().join(Npm.get_lock_file_name()).exists();
    let yarn = work_dir.as_ref().join(Yarn.get_lock_file_name()).exists();

    match (npm, yarn) {
        (_, true) => Yarn,
        _ => Npm,
    }
}

fn create_command_items(json: PackageJson, tool: Tool) -> Vec<CommandItem> {
    json.scripts
        .iter()
        .sorted()
        .map(|(key, val)| CommandItem::new(format!("{} {}", tool.get_runner(), key), vec![val.to_string()], false))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use crate::command::item::CommandItem;
    use crate::command::package_json::Tool::{Npm, Yarn};
    use crate::command::package_json::{Tool, _parse_package_json};
    use trim_margin::MarginTrimmable;

    fn setup<P: AsRef<Path>>(work_dir: P, tool: Tool) {
        let raw = r#"
            |{
            |  "name": "foo",
            |  "scripts": {
            |    "dev": "next dev",
            |    "build": "next build && next export",
            |    "format": "prettier \"./src/**/*.{ts,tsx}\""
            |  }
            |}
        "#
        .trim()
        .trim_margin()
        .unwrap();

        let work_dir = work_dir.as_ref();
        let _ = fs::create_dir_all(work_dir);
        let _ = File::create(work_dir.join("package.json")).unwrap().write_all(raw.as_bytes());
        let _ = File::create(work_dir.join(tool.get_lock_file_name()));
    }

    fn cleanup(work_dir: &str) {
        let _ = fs::remove_dir_all(work_dir);
    }

    #[test]
    fn found_npm() {
        let work_dir = "target/build-tool-launcher/test-pj/npm-found";

        setup(work_dir, Npm);

        let act = _parse_package_json(work_dir);
        let commands = vec![
            CommandItem::new("npm run build", vec!["next build && next export"], false),
            CommandItem::new("npm run dev", vec!["next dev"], false),
            CommandItem::new("npm run format", vec![r#"prettier "./src/**/*.{ts,tsx}""#], false),
        ];

        assert_eq!(act, commands);

        cleanup(work_dir);
    }

    #[test]
    fn found_yarn() {
        let work_dir = "target/build-tool-launcher/test-pj/yarn-found";

        setup(work_dir, Yarn);

        let act = _parse_package_json(work_dir);
        let commands = vec![
            CommandItem::new("yarn build", vec!["next build && next export"], false),
            CommandItem::new("yarn dev", vec!["next dev"], false),
            CommandItem::new("yarn format", vec![r#"prettier "./src/**/*.{ts,tsx}""#], false),
        ];

        assert_eq!(act, commands);

        cleanup(work_dir);
    }

    #[test]
    fn notfound() {
        let work_dir = "target/build-tool-launcher/test-pj/package-json-notfound";

        let act = _parse_package_json(work_dir);

        assert_eq!(act, vec![]);
    }
}
