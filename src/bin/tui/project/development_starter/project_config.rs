use itertools::Itertools;

use bins::libs::item::display_item::DisplayItem;
use bins::libs::project::project_config::ProjectConfig;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ProjectItem {
    label: String,
    lines: Vec<String>,
    pub origin: ProjectConfig,
}

impl ProjectItem {
    pub fn new(label: String, lines: Vec<String>, origin: ProjectConfig) -> Self {
        Self { label, lines, origin }
    }
}

impl DisplayItem for ProjectItem {
    fn get_pane1(&self) -> String {
        self.label.clone()
    }

    fn get_pane2(&self) -> Vec<String> {
        self.lines.clone()
    }

    fn is_editable(&self) -> bool {
        true
    }
}

pub fn get_project_items(project_configs: Vec<ProjectConfig>) -> Vec<ProjectItem> {
    project_configs
        .iter()
        .map(|project_config| {
            ProjectItem::new(project_config.name(), project_config.project.tags.clone(), project_config.clone())
        })
        .collect_vec()
}
