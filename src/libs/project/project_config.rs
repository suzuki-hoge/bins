use std::env::current_dir;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use crate::libs::io::reader::read_deserializable;
use crate::libs::io::writer::{delete_file, write_serializable};

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ProjectConfig {
    pub config_path: String,
    pub work_dir_path: String,
    pub config_file_exists: bool,
    pub work_dir_exists: bool,
    pub git_exists: bool,
    pub up_exists: bool,
    pub project: Project,
}

impl ProjectConfig {
    pub fn new(config_path: String, config_file_exists: bool, project: Project) -> Self {
        let work_dir_path = config_path.split('/').rev().collect_vec()[0].replace(".yaml", "").replace('.', "/");
        let work_dir_exists = Path::new(&work_dir_path).exists();
        let git_exists = Path::new(&work_dir_path).join(".git").exists();
        let up_exists = project.build_commands.iter().any(|build_command| build_command.label == "u");
        Self { config_path, work_dir_path, config_file_exists, work_dir_exists, git_exists, up_exists, project }
    }

    pub fn name(&self) -> String {
        self.config_path.to_string().split('.').rev().collect_vec()[1].to_string()
    }

    pub fn update_tags(&mut self, tags: Vec<String>) {
        self.project.tags = tags;

        let _ = write_serializable(Path::new(&self.config_path), &self.project);
    }

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

    pub fn delete_build_command(&mut self, label: String) {
        self.update_build_command(label, None);
    }

    fn update_build_command(&mut self, label: String, lines: Option<Vec<String>>) {
        let label = label.replace("bb ", "");

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

        let _ = write_serializable(Path::new(&self.config_path), &self.project);
    }

    pub fn generate(&mut self) -> bool {
        if self.config_file_exists {
            false
        } else {
            let _ = write_serializable(Path::new(&self.config_path), &self.project);
            true
        }
    }

    pub fn delete(&self) {
        let _ = delete_file(Path::new(&self.config_path));
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct Project {
    pub tags: Vec<String>,
    pub build_commands: Vec<BuildCommand>,
}

impl Project {
    fn empty() -> Self {
        Self { tags: vec![], build_commands: vec![] }
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
pub struct BuildCommand {
    pub label: String,
    pub lines: Vec<String>,
}

pub fn parse_project_configs() -> anyhow::Result<Vec<ProjectConfig>> {
    let bins_dir = PathBuf::from(std::env::var("HOME")?);
    let mut project_configs = vec![];
    for e in read_dir(&bins_dir.join(".bins-project-config"))? {
        let path = e.unwrap().path();
        let work_dir = path.file_name().unwrap().to_str().unwrap().replace(".yaml", "").replace('.', "/");
        if !work_dir.ends_with("DS_Store") {
            project_configs.push(_parse_project_config(&bins_dir, &PathBuf::from(work_dir))?);
        }
    }
    Ok(project_configs)
}

pub fn parse_project_config() -> anyhow::Result<ProjectConfig> {
    let bins_dir = PathBuf::from(std::env::var("HOME")?);
    let work_dir = current_dir()?;

    _parse_project_config(&bins_dir, &work_dir)
}

fn _parse_project_config(bins_dir: &Path, work_dir: &Path) -> anyhow::Result<ProjectConfig> {
    let work_dir_dot = work_dir.display().to_string().replace('/', ".");
    let yaml_path = bins_dir.join(".bins-project-config").join(format!("{work_dir_dot}.yaml"));

    let config_path = yaml_path.display().to_string();

    match read_deserializable(&yaml_path) {
        Ok(yaml) => Ok(ProjectConfig::new(config_path, true, yaml)),
        Err(_) => Ok(ProjectConfig::new(config_path, false, Project::empty())),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    use itertools::Itertools;
    use trim_margin::MarginTrimmable;

    use crate::libs::project::project_config::{BuildCommand, Project, _parse_project_config};

    fn setup(bins_dir: &Path) {
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

        let _ = fs::create_dir_all(bins_dir);
        let _ = fs::create_dir_all(bins_dir.join(".bins-project-config"));
        let _ = File::create(bins_dir.join(".bins-project-config").join(".path.front.yaml"))
            .unwrap()
            .write_all(raw.as_bytes());
    }

    fn cleanup(bins_dir: &PathBuf) {
        let _ = fs::remove_dir_all(bins_dir);
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

        let bins_dir = PathBuf::from("target/build-tool-launcher/test-pj/project-config-found");

        setup(&bins_dir);

        let work_dir = PathBuf::from("/path/front");

        // read

        let mut sut = _parse_project_config(&bins_dir, &work_dir).unwrap();

        // assert

        let project = exp(vec!["react", "next"], vec![("up", "yarn dev")]);
        assert_eq!(sut.project, project);

        // insert + update

        sut.upsert_build_command("bb up".to_string(), vec!["yarn build && yarn start".to_string()]);
        sut.upsert_build_command("bb down".to_string(), vec!["yarn stop".to_string()]);

        // assert

        let project = exp(vec!["react", "next"], vec![("up", "yarn build && yarn start"), ("down", "yarn stop")]);
        assert_eq!(sut.project, project);

        // delete

        sut.delete_build_command("bb up".to_string());

        // assert

        let project = exp(vec!["react", "next"], vec![("down", "yarn stop")]);
        assert_eq!(sut.project, project);

        // clean

        cleanup(&bins_dir);
    }

    #[test]
    fn notfound() {
        let bins_dir = PathBuf::from("target/build-tool-launcher/test-pj/project-config-notfound");

        let work_dir = PathBuf::from("/path/front");

        let sut = _parse_project_config(&bins_dir, &work_dir).unwrap();

        let project = exp(vec![], vec![]);
        assert_eq!(sut.project, project);
    }
}
