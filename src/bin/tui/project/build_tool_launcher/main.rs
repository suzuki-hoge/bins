extern crate bins;

use std::env::current_dir;
use std::path::{Path, PathBuf};

use itertools::Itertools;

use bins::libs::io::writer::output_or_exit;
use bins::libs::item::display_item::DisplayItem;
use bins::libs::launcher::crossterm_launcher::launch;
use bins::libs::project::project_mapper::parse_project_mapper;

use crate::command::makefile::parse_makefile;
use crate::command::package_json::parse_package_json;
use crate::command::project_mapper::parse_project_mapper_current_config;

mod command;
mod runner;
mod ui;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect_vec();

    let home = PathBuf::from(std::env::var("HOME")?);
    let work_dir = current_dir()?;

    if args.len() == 1 {
        select(&home, &work_dir)
    } else if args.len() == 2 {
        match args[1].as_str() {
            "-p" => push(&home, &work_dir),
            s => selected(&home, &work_dir, s.to_string()),
        }
    } else {
        output_or_exit("echo invalid args")
    }
}

fn select(home: &Path, work_dir: &Path) -> anyhow::Result<()> {
    let project_mapper_current_config = parse_project_mapper_current_config(home, work_dir);

    let commands = vec![parse_makefile(work_dir), parse_package_json(work_dir)].into_iter().flatten().collect_vec();

    match launch(|terminal| runner::run(terminal, project_mapper_current_config.clone(), commands.clone())) {
        Ok(items) => output_or_exit(items.iter().map(|item| item.get_pane1()).join("\n")),
        Err(e) => output_or_exit(format!("echo {e}")),
    }
}

fn selected(home: &Path, work_dir: &Path, arg: String) -> anyhow::Result<()> {
    match parse_project_mapper(home, work_dir).get_build_command_lines(arg) {
        Some(lines) => output_or_exit(lines.join("\n")),
        None => output_or_exit("echo no such command"),
    }
}

fn push(home: &Path, work_dir: &Path) -> anyhow::Result<()> {
    if parse_project_mapper(home, work_dir).generate() {
        output_or_exit("echo generated")
    } else {
        output_or_exit("echo already generated")
    }
}
