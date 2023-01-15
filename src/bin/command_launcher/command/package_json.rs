use crate::command::package_json::Tool::{Npm, Yarn};
use crate::command::parsed_command::{ParsedCommand, ParsedContent};
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

    fn get_file(&self) -> &str {
        match self {
            Npm => "package-lock.json",
            Yarn => "yarn.lock",
        }
    }
}

pub fn parse_package_json(dir_path: String) -> ParsedCommand {
    match read_file(dir_path.clone()) {
        Ok(json) => parse(json, find_tool(dir_path)),
        Err(_) => ParsedCommand::empty(),
    }
}

fn read_file(dir_path: String) -> Result<PackageJson, ()> {
    let file = File::open(format!("{}/package.json", dir_path)).map_err(|_| ())?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).map_err(|_| ())
}

fn find_tool(dir_path: String) -> Tool {
    let npm = Path::new(&format!("{}/{}", dir_path, Npm.get_file())).exists();
    let yarn = Path::new(&format!("{}/{}", dir_path, Yarn.get_file())).exists();
    match (npm, yarn) {
        (_, true) => Yarn,
        _ => Npm,
    }
}

fn parse(json: PackageJson, tool: Tool) -> ParsedCommand {
    ParsedCommand::new(
        json.scripts
            .iter()
            .sorted()
            .map(|(key, val)| ParsedContent::new(format!("{} {}", tool.get_runner(), key), vec![val.to_string()]))
            .collect_vec(),
    )
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;

    use std::io::Write;

    use crate::command::package_json::Tool::{Npm, Yarn};
    use crate::command::package_json::{parse_package_json, Tool};
    use crate::command::parsed_command::ParsedCommand;
    use crate::ParsedContent;
    use trim_margin::MarginTrimmable;

    fn setup(dir_path: &str, tool: Tool) {
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
        let _ = File::create(format!("{}/package.json", dir_path)).unwrap().write_all(raw.as_bytes());
        let _ = File::create(format!("{}/{}", dir_path, tool.get_file()));
    }

    fn cleanup(dir_path: &str) {
        let _ = fs::remove_dir_all(dir_path);
    }

    #[test]
    fn found_npm() {
        let dir_path = "target/command-launcher/test-pj/npm-found";

        setup(dir_path, Npm);

        let sut = parse_package_json(dir_path.to_string());
        let contents = vec![
            ParsedContent::new("npm run build".to_string(), vec!["next build && next export".to_string()]),
            ParsedContent::new("npm run dev".to_string(), vec!["next dev".to_string()]),
            ParsedContent::new("npm run format".to_string(), vec!["prettier \"./src/**/*.{ts,tsx}\"".to_string()]),
        ];

        assert_eq!(sut, ParsedCommand::new(contents));

        cleanup(dir_path);
    }

    #[test]
    fn found_yarn() {
        let dir_path = "target/command-launcher/test-pj/yarn-found";

        setup(dir_path, Yarn);

        let sut = parse_package_json(dir_path.to_string());
        let contents = vec![
            ParsedContent::new("yarn build".to_string(), vec!["next build && next export".to_string()]),
            ParsedContent::new("yarn dev".to_string(), vec!["next dev".to_string()]),
            ParsedContent::new("yarn format".to_string(), vec!["prettier \"./src/**/*.{ts,tsx}\"".to_string()]),
        ];

        assert_eq!(sut, ParsedCommand::new(contents));

        cleanup(dir_path);
    }

    #[test]
    fn notfound() {
        let dir_path = "target/command-launcher/test-pj/package-json-notfound";

        let sut = parse_package_json(dir_path.to_string());

        assert_eq!(sut, ParsedCommand::empty());
    }
}
