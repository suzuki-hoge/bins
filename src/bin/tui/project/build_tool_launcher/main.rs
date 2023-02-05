extern crate bins;

use itertools::Itertools;

use bins::libs::io::writer::output_or_exit;
use bins::libs::item::display_item::DisplayItem;
use bins::libs::launcher::crossterm_launcher::launch;
use bins::libs::project::project_config::parse_project_config;

use crate::command::makefile::parse_makefile;
use crate::command::package_json::parse_package_json;

mod command;
mod runner;
mod ui;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect_vec();

    if args.len() == 1 {
        select()
    } else if args.len() == 2 {
        match args[1].as_str() {
            "--generate" => generate(),
            label => selected(label.to_string()),
        }
    } else {
        output_or_exit("echo invalid args")
    }
}

fn select() -> anyhow::Result<()> {
    let project_config = parse_project_config()?;

    let command_items = vec![parse_makefile()?, parse_package_json()?].into_iter().flatten().collect_vec();

    match launch(|terminal| runner::run(terminal, project_config.clone(), command_items.clone())) {
        Ok(items) => output_or_exit(items.iter().map(|item| item.get_pane2().join("\n")).join("\n")),
        Err(e) => output_or_exit(format!("echo {e}")),
    }
}

fn selected(label: String) -> anyhow::Result<()> {
    match parse_project_config()?.get_build_command_lines(label) {
        Some(lines) => output_or_exit(lines.join("\n")),
        None => output_or_exit("echo no such command"),
    }
}

fn generate() -> anyhow::Result<()> {
    if parse_project_config()?.generate() {
        output_or_exit("echo generated")
    } else {
        output_or_exit("echo already generated")
    }
}
