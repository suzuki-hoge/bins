use std::fs::read_dir;
use std::path::{Path, PathBuf};

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use bins::fuzzy::core::item::Item;
use bins::git::config::{get_git_config, GitConfig};
use bins::io::file::read_deserializable;

pub fn parse_project_configs() -> Vec<CommandItem> {
    let home_dir = PathBuf::from(std::env::var("HOME").unwrap());
    let mut command_items = vec![];
    for e in read_dir(home_dir.join(".bins-project-config")).unwrap() {
        let path = e.unwrap().path();
        let work_dir = path.file_name().unwrap().to_str().unwrap().replace(".yaml", "").replace('.', "/");
        if !work_dir.ends_with("DS_Store") {
            if let Some(item) = _parse_project_config(&home_dir, work_dir) {
                command_items.push(item);
            }
        }
    }
    command_items.sort_by_key(|item| item.get_line());
    command_items
}

fn _parse_project_config<P1: AsRef<Path>, P2: AsRef<Path>>(bins_dir: P1, work_dir: P2) -> Option<CommandItem> {
    let work_dir_dot = work_dir.as_ref().display().to_string().replace('/', ".");
    let yaml_path = bins_dir.as_ref().join(".bins-project-config").join(format!("{work_dir_dot}.yaml"));

    match read_deserializable(yaml_path) {
        Ok(project_config) => Some(CommandItem::new(work_dir.as_ref().display().to_string(), project_config)),
        Err(_) => None,
    }
}

#[derive(Serialize, Deserialize)]
struct ProjectConfig {
    pub tags: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct CommandItem {
    pub work_dir: String,
    name: String,
    tags: Vec<String>,
    git_config: Option<GitConfig>,
}

impl CommandItem {
    fn new(work_dir: String, project_config: ProjectConfig) -> Self {
        let name = work_dir.split('/').rev().collect_vec()[0].to_string();
        let git_config = get_git_config(&work_dir);
        Self { work_dir, name, tags: project_config.tags, git_config }
    }

    pub fn is_git_hub_enabled(&self) -> bool {
        self.git_config.is_some()
    }
}

impl Item for CommandItem {
    fn get_line(&self) -> String {
        self.name.to_string()
    }

    fn get_preview(&self) -> Vec<String> {
        vec![
            format!("tags: {}", self.tags.join(", ")),
            if let Some(git_config) = &self.git_config {
                format!("git: {}/{}", git_config.owner, git_config.repo)
            } else {
                "git:".to_string()
            },
        ]
    }
}
