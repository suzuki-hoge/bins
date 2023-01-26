use crate::command::command_item::CommandItem;
use crate::command::package_json::Tool::{Npm, Yarn};
use itertools::Itertools;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

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

pub fn parse_package_json(dir_path: &Path) -> Vec<CommandItem> {
    match read_file(dir_path) {
        Ok(json) => parse(json, find_tool(dir_path)),
        Err(_) => vec![],
    }
}

fn read_file(dir_path: &Path) -> anyhow::Result<PackageJson, ()> {
    let file = File::open(dir_path.join("package.json")).map_err(|_| ())?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|_| ())
}

fn find_tool(dir_path: &Path) -> Tool {
    let npm = dir_path.join(Npm.get_lock_file_name()).exists();
    let yarn = dir_path.join(Yarn.get_lock_file_name()).exists();
    match (npm, yarn) {
        (_, true) => Yarn,
        _ => Npm,
    }
}

fn parse(json: PackageJson, tool: Tool) -> Vec<CommandItem> {
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
    use std::path::{Path, PathBuf};

    use crate::command::command_item::CommandItem;
    use crate::command::package_json::Tool::{Npm, Yarn};
    use crate::command::package_json::{parse_package_json, Tool};
    use trim_margin::MarginTrimmable;

    fn setup(dir_path: &Path, tool: Tool) {
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

        let _ = fs::create_dir_all(dir_path);
        let _ = File::create(dir_path.join("package.json")).unwrap().write_all(raw.as_bytes());
        let _ = File::create(dir_path.join(tool.get_lock_file_name()));
    }

    fn cleanup(dir_path: &PathBuf) {
        let _ = fs::remove_dir_all(dir_path);
    }

    #[test]
    fn found_npm() {
        let dir_path = PathBuf::from("target/build-tool-launcher/test-pj/npm-found");

        setup(&dir_path, Npm);

        let sut = parse_package_json(&dir_path);
        let commands = vec![
            CommandItem::new("npm run build".to_string(), vec!["next build && next export".to_string()]),
            CommandItem::new("npm run dev".to_string(), vec!["next dev".to_string()]),
            CommandItem::new("npm run format".to_string(), vec!["prettier \"./src/**/*.{ts,tsx}\"".to_string()]),
        ];

        assert_eq!(sut, commands);

        cleanup(&dir_path);
    }

    #[test]
    fn found_yarn() {
        let dir_path = PathBuf::from("target/build-tool-launcher/test-pj/yarn-found");

        setup(&dir_path, Yarn);

        let sut = parse_package_json(&dir_path);
        let commands = vec![
            CommandItem::new("yarn build".to_string(), vec!["next build && next export".to_string()]),
            CommandItem::new("yarn dev".to_string(), vec!["next dev".to_string()]),
            CommandItem::new("yarn format".to_string(), vec!["prettier \"./src/**/*.{ts,tsx}\"".to_string()]),
        ];

        assert_eq!(sut, commands);

        cleanup(&dir_path);
    }

    #[test]
    fn notfound() {
        let dir_path = PathBuf::from("target/build-tool-launcher/test-pj/package-json-notfound");

        let sut = parse_package_json(&dir_path);

        assert_eq!(sut, vec![]);
    }
}
