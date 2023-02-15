use crate::command::command_item::CommandItem;
use bins::libs::project::project_config::ProjectConfig;
use itertools::Itertools;

pub fn get_command_items(project_config: &ProjectConfig) -> Vec<CommandItem> {
    project_config
        .project
        .build_commands
        .iter()
        .map(|build_command| {
            CommandItem::new_editable(format!("bb {}", build_command.label.clone()), build_command.lines.clone())
        })
        .collect_vec()
}
