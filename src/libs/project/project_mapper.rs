use std::cmp::{max, min};
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::BufReader;
use std::path::Path;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct ProjectMapper {
    path: String,
    exists: bool,
    pub project: Project,
}

impl ProjectMapper {
    pub fn get_build_command_lines(&self, label: String) -> Option<Vec<String>> {
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
pub struct Project {
    pub tags: Vec<String>,
    pub build_commands: Vec<BuildCommand>,
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct BuildCommand {
    pub label: String,
    pub lines: Vec<String>,
}

pub fn parse_project_mapper(yaml_dir_path: &Path, current_dir_path: &Path) -> ProjectMapper {
    let current_dir_path = current_dir_path.display().to_string().replace('/', ".");
    let path =
        yaml_dir_path.join(".bins-project-mapper").join(format!("{current_dir_path}.yaml")).display().to_string();

    match read_file(&path) {
        Ok(yaml) => ProjectMapper { path, exists: true, project: yaml },
        Err(_) => ProjectMapper { path, exists: false, project: Project { tags: vec![], build_commands: vec![] } },
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

    use itertools::Itertools;
    use trim_margin::MarginTrimmable;

    use crate::libs::project::project_mapper::{parse_project_mapper, BuildCommand, Project};

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

    fn exp(tags: Vec<&str>, commands: Vec<(&str, &str)>) -> Project {
        Project {
            tags: tags.iter().map(|s| s.to_string()).collect_vec(),
            build_commands: commands
                .iter()
                .map(|(label, command)| BuildCommand { label: label.to_string(), lines: vec![command.to_string()] })
                .collect_vec(),
        }
    }

    #[test]
    fn found() {
        // setup

        let yaml_dir_path = PathBuf::from("target/build-tool-launcher/test-pj/project-mapper-found");

        setup(&yaml_dir_path);

        let current_dir_path = PathBuf::from("/path/front");

        // read

        let mut sut = parse_project_mapper(&yaml_dir_path, &current_dir_path);

        // assert

        let project = exp(vec!["react", "next"], vec![("up", "yarn dev")]);
        assert_eq!(sut.project, project);

        // insert + update

        sut.upsert_build_command("bb up".to_string(), vec!["yarn build && yarn start".to_string()]);
        sut.upsert_build_command("bb down".to_string(), vec!["yarn stop".to_string()]);

        // read

        let mut sut = parse_project_mapper(&yaml_dir_path, &current_dir_path);

        // assert

        let project = exp(vec!["react", "next"], vec![("up", "yarn build && yarn start"), ("down", "yarn stop")]);
        assert_eq!(sut.project, project);

        // delete

        sut.delete_build_command("bb up".to_string());

        // read

        let sut = parse_project_mapper(&yaml_dir_path, &current_dir_path);

        // assert

        let project = exp(vec!["react", "next"], vec![("down", "yarn stop")]);
        assert_eq!(sut.project, project);

        // clean

        cleanup(&yaml_dir_path);
    }

    #[test]
    fn notfound() {
        let yaml_dir_path = PathBuf::from("target/build-tool-launcher/test-pj/project-mapper-notfound");

        let current_dir_path = PathBuf::from("/path/front");

        let sut = parse_project_mapper(&yaml_dir_path, &current_dir_path);

        let project = exp(vec![], vec![]);
        assert_eq!(sut.project, project);
    }
}
