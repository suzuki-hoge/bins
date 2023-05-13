use crate::item::command_item::CommandItem;
use crate::item::makefile::parse_makefile;
use crate::item::package_json::parse_package_json;
use crate::item::project_config::generate_project_config as generate;
use crate::item::project_config::get_project_config_path as get_path;
use crate::item::project_config::parse_project_config;
use std::path::PathBuf;

mod command_item;
mod makefile;
mod package_json;
mod project_config;

pub fn gather() -> Vec<CommandItem> {
    vec![parse_makefile(), parse_package_json(), parse_project_config()].into_iter().flatten().collect()
}

pub fn get_project_config() -> Vec<CommandItem> {
    parse_project_config()
}

pub fn get_project_config_path() -> PathBuf {
    get_path()
}

pub fn generate_project_config() -> anyhow::Result<()> {
    generate()
}
