use crate::command::item::CommandItem;
use bins::io::file::{read_deserializable, write_serializable};
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::path::{Path, PathBuf};

pub fn parse_project_config() -> Vec<CommandItem> {
    let home_dir = std::env::var("HOME").unwrap();
    let work_dir = current_dir().unwrap();

    _parse_project_config(home_dir, work_dir)
}

fn _parse_project_config<P1: AsRef<Path>, P2: AsRef<Path>>(home_dir: P1, work_dir: P2) -> Vec<CommandItem> {
    let work_dir_dot = work_dir.as_ref().display().to_string().replace('/', ".");
    let config_path = home_dir.as_ref().join(".bins-project-config").join(format!("{work_dir_dot}.yaml"));

    match read_deserializable(config_path) {
        Ok(lines) => create_command_items(lines),
        Err(_) => vec![],
    }
}

fn create_command_items(project_config: ProjectConfig) -> Vec<CommandItem> {
    project_config
        .build_commands
        .into_iter()
        .map(|build_command| CommandItem::new(format!("bb {}", build_command.label), build_command.lines, true))
        .collect()
}

pub fn get_project_config_path() -> PathBuf {
    let home_dir = PathBuf::from(std::env::var("HOME").unwrap());
    let work_dir = current_dir().unwrap();
    let work_dir_dot = work_dir.display().to_string().replace('/', ".");
    home_dir.join(".bins-project-config").join(format!("{work_dir_dot}.yaml"))
}

pub fn generate_project_config() -> anyhow::Result<()> {
    let project_config = ProjectConfig { build_commands: vec![] };
    write_serializable(get_project_config_path(), &project_config)
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct ProjectConfig {
    pub build_commands: Vec<BuildCommand>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct BuildCommand {
    pub label: String,
    pub lines: Vec<String>,
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::Path;

    use crate::command::item::CommandItem;
    use crate::command::project_config::_parse_project_config;
    use trim_margin::MarginTrimmable;

    fn setup<P: AsRef<Path>>(home_dir: P) {
        let raw = r#"
            |tags: [react, next]
            |build_commands:
            |- label: up
            |  lines:
            |  - yarn build
            |  - yarn start
            |- label: dev
            |  lines:
            |  - yarn dev
        "#
        .trim()
        .trim_margin()
        .unwrap();

        let home_dir = home_dir.as_ref();
        let _ = fs::create_dir_all(home_dir);
        let _ = fs::create_dir_all(home_dir.join(".bins-project-config"));
        let _ = File::create(home_dir.join(".bins-project-config").join(".path.front.yaml"))
            .unwrap()
            .write_all(raw.as_bytes());
    }

    fn cleanup(bins_dir: &str) {
        let _ = fs::remove_dir_all(bins_dir);
    }

    #[test]
    fn found() {
        let home_dir = "target/build-tool-launcher/test-pj/project-config-found";

        setup(home_dir);

        let work_dir = "/path/front";

        let act = _parse_project_config(home_dir, work_dir);
        let commands = vec![
            CommandItem::new("bb up", vec!["yarn build", "yarn start"], true),
            CommandItem::new("bb dev", vec!["yarn dev"], true),
        ];

        assert_eq!(act, commands);

        cleanup(home_dir);
    }

    #[test]
    fn notfound() {
        let home_dir = "target/build-tool-launcher/test-pj/project-config-notfound";

        let work_dir = "/path/front";

        let act = _parse_project_config(home_dir, work_dir);

        assert_eq!(act, vec![]);
    }
}
