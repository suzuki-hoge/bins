extern crate bins;

use itertools::Itertools;
use structopt::StructOpt;

use bins::libs::io::writer::{stderr, stdout};
use bins::libs::launcher::crossterm_launcher::launch;
use bins::libs::project::project_config::parse_project_config;

use crate::command::makefile::parse_makefile;
use crate::command::package_json::parse_package_json;

mod command;
mod runner;
mod ui;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "g", long = "--generate", help = "generate empty config")]
    generate: bool,

    #[structopt(name = "command_name", help = "run specified command instantly")]
    name: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    match (opt.generate, opt.name) {
        (true, _) => generate(),
        (false, Some(name)) => run_instantly(name),
        (false, None) => launch_selector(),
    }
}

fn launch_selector() -> anyhow::Result<()> {
    let project_config = parse_project_config()?;

    let command_items = vec![parse_makefile()?, parse_package_json()?].into_iter().flatten().collect_vec();

    match launch(|terminal| runner::run(terminal, project_config.clone(), command_items.clone())) {
        Ok(items) => stdout(items.iter().map(|item| item.get_runnable()).join("\n")),
        Err(e) => stderr(e),
    }
}

fn run_instantly(label: String) -> anyhow::Result<()> {
    match parse_project_config()?.get_build_command_lines(label) {
        Some(lines) => stdout(lines.join("\n")),
        None => stderr("no such command"),
    }
}

fn generate() -> anyhow::Result<()> {
    if parse_project_config()?.generate() {
        stdout("echo generated")
    } else {
        stderr("already generated")
    }
}
