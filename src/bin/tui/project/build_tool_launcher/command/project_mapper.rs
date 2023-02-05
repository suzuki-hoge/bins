use std::path::Path;

use itertools::Itertools;

use bins::libs::project::project_mapper::{parse_project_mapper, ProjectMapper};

use crate::command::command_item::CommandItem;

#[derive(Clone)]
pub struct ProjectMapperCurrentConfig {
    project_mapper: ProjectMapper,
}

impl ProjectMapperCurrentConfig {
    pub fn get_commands(&self) -> Vec<CommandItem> {
        self.project_mapper
            .project
            .build_commands
            .iter()
            .sorted_by_key(|build_command| build_command.label.clone())
            .map(|build_command| {
                CommandItem::new_editable(format!("bb {}", build_command.label), build_command.lines.clone())
            })
            .collect_vec()
    }

    pub fn upsert_build_command(&mut self, label: String, lines: Vec<String>) {
        self.project_mapper.upsert_build_command(label, lines);
    }

    pub fn delete_build_command(&mut self, label: String) -> bool {
        self.project_mapper.delete_build_command(label)
    }
}

pub fn parse_project_mapper_current_config(
    yaml_dir_path: &Path,
    current_dir_path: &Path,
) -> ProjectMapperCurrentConfig {
    ProjectMapperCurrentConfig { project_mapper: parse_project_mapper(yaml_dir_path, current_dir_path) }
}
