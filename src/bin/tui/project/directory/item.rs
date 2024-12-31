use std::path::PathBuf;

use itertools::Itertools;
use serde::{Deserialize, Serialize};

use bins::fuzzy::core::item::Item;
use bins::git::config::{get_git_config, GitConfig};
use bins::io::file::{get_bins_dir, read_deserializable};

pub fn get_config_path() -> PathBuf {
    get_bins_dir().join("projects.yml")
}

pub fn parse_items() -> Vec<ParsedItem> {
    let rows: Vec<YamlRow> = read_deserializable(get_config_path()).unwrap();

    rows.into_iter().map(ParsedItem::from).collect()
}

#[derive(Serialize, Deserialize)]
struct YamlRow {
    path: String,
    tags: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct ParsedItem {
    pub path: String,
    tags: Vec<String>,
    name: String,
    git_config: Option<GitConfig>,
}

impl From<YamlRow> for ParsedItem {
    fn from(def: YamlRow) -> Self {
        let name = def.path.split('/').rev().collect_vec()[0].to_string();
        let git_config = get_git_config(&def.path);
        Self { path: def.path, tags: def.tags, name, git_config }
    }
}

impl ParsedItem {
    pub fn is_git_hub_enabled(&self) -> bool {
        self.git_config.is_some()
    }
}

impl Item for ParsedItem {
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
