use std::collections::HashMap;
use std::env::current_dir;
use std::path::Path;

use itertools::Itertools;
use serde::Deserialize;

use bins::libs::io::reader::read_deserializable;

use crate::command::command_item::CommandItem;
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

pub fn parse_package_json() -> anyhow::Result<Vec<CommandItem>> {
    let work_dir = current_dir()?;

    _parse_package_json(work_dir)
}

fn _parse_package_json<P: AsRef<Path>>(work_dir: P) -> anyhow::Result<Vec<CommandItem>> {
    match read_deserializable(work_dir.as_ref().join("package.json")) {
        Ok(json) => Ok(create_command_items(json, find_tool(work_dir))),
        Err(_) => Ok(vec![]),
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
        .map(|(key, val)| CommandItem::new(format!("{} {}", tool.get_runner(), key), vec![val.to_string()]))
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use trim_margin::MarginTrimmable;

    use crate::command::command_item::CommandItem;
    use crate::command::package_json::Tool::{Npm, Yarn};
    use crate::command::package_json::{Tool, _parse_package_json};

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

        let act = _parse_package_json(work_dir).unwrap();
        let commands = vec![
            CommandItem::new("npm run build".to_string(), vec!["next build && next export".to_string()]),
            CommandItem::new("npm run dev".to_string(), vec!["next dev".to_string()]),
            CommandItem::new("npm run format".to_string(), vec!["prettier \"./src/**/*.{ts,tsx}\"".to_string()]),
        ];

        assert_eq!(act, commands);

        cleanup(work_dir);
    }

    #[test]
    fn found_yarn() {
        let work_dir = "target/build-tool-launcher/test-pj/yarn-found";

        setup(work_dir, Yarn);

        let act = _parse_package_json(work_dir).unwrap();
        let commands = vec![
            CommandItem::new("yarn build".to_string(), vec!["next build && next export".to_string()]),
            CommandItem::new("yarn dev".to_string(), vec!["next dev".to_string()]),
            CommandItem::new("yarn format".to_string(), vec!["prettier \"./src/**/*.{ts,tsx}\"".to_string()]),
        ];

        assert_eq!(act, commands);

        cleanup(work_dir);
    }

    #[test]
    fn notfound() {
        let work_dir = "target/build-tool-launcher/test-pj/package-json-notfound";

        let act = _parse_package_json(work_dir).unwrap();

        assert_eq!(act, vec![]);
    }
}
