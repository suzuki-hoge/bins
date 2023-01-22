extern crate bins;

use crate::command::makefile::parse_makefile;
use crate::command::package_json::parse_package_json;
use bins::libs::io::writer::output_or_exit;
use bins::libs::item::display_item::DisplayItem;
use bins::libs::launcher::crossterm_launcher::launch;

use itertools::Itertools;
use std::env::current_dir;

mod command;
mod runner;
mod ui;

fn main() -> anyhow::Result<()> {
    let dir_path = current_dir()?;
    let commands = vec![parse_makefile(&dir_path), parse_package_json(&dir_path)].into_iter().flatten().collect_vec();

    match launch(|terminal| runner::run(terminal, commands.clone())) {
        Ok(items) => output_or_exit(items.iter().map(|item| item.get_pane1()).join("\n")),
        Err(e) => output_or_exit(e),
    }
}
