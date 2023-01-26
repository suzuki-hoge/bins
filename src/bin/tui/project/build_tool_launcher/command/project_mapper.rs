use std::cmp::{max, min};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::Path;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::command::command_item::CommandItem;

#[derive(Clone)]
pub struct ProjectMapperCurrentConfig {
    path: String,
    exists: bool,
    project: Project,
}

impl ProjectMapperCurrentConfig {
    pub fn get_commands(&self) -> Vec<CommandItem> {
        self.project
            .build_commands
            .iter()
            .sorted()
            .map(|build_command| {
                CommandItem::new_editable(format!("bb {}", build_command.label), build_command.lines.clone())
            })
            .collect_vec()
    }

    pub fn get_lines(&self, label: String) -> Option<Vec<String>> {
        self.project
            .build_commands
            .iter()
            .find(|build_command| build_command.label == label)
            .map(|build_command| build_command.lines.clone())
    }

    pub fn upsert_build_command(&mut self, label: String, lines: Vec<String>) {
        self.update_build_command(label, Some(lines));
    }

    pub fn delete_build_command(&mut self, label: String) -> bool {
        self.update_build_command(label, None) == 1
    }

    pub fn generate(&mut self) -> bool {
        if self.exists {
            false
        } else {
            self.update_build_command("".to_string(), None);
            true
        }
    }

    fn update_build_command(&mut self, label: String, lines: Option<Vec<String>>) -> usize {
        let label = label.replace("bb ", "");

        let origin = self.project.build_commands.len();
        self.project.build_commands = self
            .project
            .build_commands
            .clone()
            .into_iter()
            .filter(|build_command| build_command.label != label)
            .collect_vec();
        if let Some(lines) = lines {
            self.project.build_commands.push(BuildCommand { label, lines });
        }
        let updated = self.project.build_commands.len();

        let _ = fs::remove_file(&self.path);
        let file = OpenOptions::new().write(true).create(true).open(&self.path).unwrap();
        let _ = serde_yaml::to_writer(file, &self.project);
        max(origin, updated) - min(origin, updated)
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
struct Project {
    tags: Vec<String>,
    build_commands: Vec<BuildCommand>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
struct BuildCommand {
    label: String,
    lines: Vec<String>,
}

pub fn parse_project_mapper_current_config(
    yaml_dir_path: &Path,
    current_dir_path: &Path,
) -> ProjectMapperCurrentConfig {
    let current_dir_path = current_dir_path.display().to_string().replace('/', ".");
    let path =
        yaml_dir_path.join(".bins-project-mapper").join(format!("{}.yaml", current_dir_path)).display().to_string();

    match read_file(&path) {
        Ok(yaml) => ProjectMapperCurrentConfig { path, exists: true, project: yaml },
        Err(_) => ProjectMapperCurrentConfig {
            path,
            exists: false,
            project: Project { tags: vec![], build_commands: vec![] },
        },
    }
}

fn read_file(path: &String) -> anyhow::Result<Project, ()> {
    let file = File::open(path).map_err(|_| ())?;
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader).map_err(|_| ())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    use trim_margin::MarginTrimmable;

    use crate::command::command_item::CommandItem;
    use crate::command::project_mapper::parse_project_mapper_current_config;

    fn setup(yaml_dir_path: &Path) {
        let raw = r#"
            |tags: [react, next]
            |build_commands:
            |- label: up
            |  lines:
            |  - yarn dev
        "#
        .trim()
        .trim_margin()
        .unwrap();

        let _ = fs::create_dir_all(yaml_dir_path);
        let _ = fs::create_dir_all(yaml_dir_path.join(".bins-project-mapper"));
        let _ = File::create(yaml_dir_path.join(".bins-project-mapper").join(".path.front.yaml"))
            .unwrap()
            .write_all(raw.as_bytes());
    }

    fn cleanup(yaml_dir_path: &PathBuf) {
        let _ = fs::remove_dir_all(yaml_dir_path);
    }

    #[test]
    fn found() {
        // setup

        let yaml_dir_path = PathBuf::from("target/build-tool-launcher/test-pj/project-mapper-found");

        setup(&yaml_dir_path);

        let current_dir_path = PathBuf::from("/path/front");

        // read

        let mut sut = parse_project_mapper_current_config(&yaml_dir_path, &current_dir_path);

        // assert

        let commands = vec![CommandItem::new_editable("bb up".to_string(), vec!["yarn dev".to_string()])];
        assert_eq!(sut.get_commands(), commands);

        // insert + update

        sut.upsert_build_command("bb up".to_string(), vec!["yarn build && yarn start".to_string()]);
        sut.upsert_build_command("bb down".to_string(), vec!["yarn stop".to_string()]);

        // read

        let mut sut = parse_project_mapper_current_config(&yaml_dir_path, &current_dir_path);

        // assert

        let commands = vec![
            CommandItem::new_editable("bb down".to_string(), vec!["yarn stop".to_string()]),
            CommandItem::new_editable("bb up".to_string(), vec!["yarn build && yarn start".to_string()]),
        ];
        assert_eq!(sut.get_commands(), commands);

        // delete

        sut.delete_build_command("bb up".to_string());

        // read

        let sut = parse_project_mapper_current_config(&yaml_dir_path, &current_dir_path);

        // assert

        let commands = vec![CommandItem::new_editable("bb down".to_string(), vec!["yarn stop".to_string()])];
        assert_eq!(sut.get_commands(), commands);

        // clean

        cleanup(&yaml_dir_path);
    }

    #[test]
    fn notfound() {
        let yaml_dir_path = PathBuf::from("target/build-tool-launcher/test-pj/project-mapper-notfound");

        let current_dir_path = PathBuf::from("/path/front");

        let sut = parse_project_mapper_current_config(&yaml_dir_path, &current_dir_path);

        assert_eq!(sut.get_commands(), vec![]);
    }
}
