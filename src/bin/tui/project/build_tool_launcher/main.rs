use itertools::Itertools;
use structopt::StructOpt;
use tui::layout::{Constraint, Direction};
use Constraint::Percentage;
use Direction::Horizontal;

use bins::fuzzy::FuzzyBuilder;
use bins::io::stdin::{stderr, stdout};

use crate::command::{gather, generate_project_config, get_project_config, get_project_config_path};

mod command;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "e", long = "--edit", help = "edit project config")]
    edit: bool,

    #[structopt(name = "command_label", help = "run specified command instantly")]
    label: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    match (opt.edit, opt.label) {
        (true, _) => edit(),
        (_, Some(label)) => run(label),
        (_, None) => fuzzy(),
    }
}

fn edit() -> anyhow::Result<()> {
    let path = get_project_config_path();
    if !path.exists() {
        generate_project_config()?
    }
    stdout(format!("vi {}", path.display()))
}

fn run(label: String) -> anyhow::Result<()> {
    match get_project_config().into_iter().find(|item| item.is_bb_match(&label)) {
        Some(item) => stdout(item.as_runnable()),
        None => stderr("no such command"),
    }
}

fn fuzzy() -> anyhow::Result<()> {
    let items = gather();

    let (items, _) = FuzzyBuilder::pane(items, Horizontal, Percentage(30)).default_preview().build().run()?;

    stdout(items.into_iter().map(|item| item.as_runnable()).join("\n"))
}
