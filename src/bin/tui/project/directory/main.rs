use std::path::Path;
use std::process::ExitCode;
use structopt::StructOpt;
use tui::layout::{Constraint, Direction};
use Constraint::Percentage;
use Direction::Horizontal;

use bins::fuzzy::FuzzyBuilder;
use bins::io::exit::exit_ok;

use crate::item::{get_config_path, parse_items};

mod item;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "out", help = "stdout file path")]
    out: String,

    #[structopt(short = "e", long = "--edit", help = "edit config")]
    edit: bool,
}

fn main() -> anyhow::Result<ExitCode> {
    let opt = Opt::from_args();

    match opt.edit {
        true => edit(&opt.out),
        false => fuzzy(&opt.out),
    }
}

fn edit<P: AsRef<Path>>(out: P) -> anyhow::Result<ExitCode> {
    exit_ok(out, format!("vi {}", get_config_path().display()))
}

fn fuzzy<P: AsRef<Path>>(out: P) -> anyhow::Result<ExitCode> {
    let items = parse_items();

    let (items, guide) = FuzzyBuilder::pane(items, Horizontal, Percentage(30))
        .default_preview()
        .guide(vec!["cd", "edit", "git"], vec![0])
        .build()
        .run()?;

    let mut commands = vec![];

    for item in items {
        if guide.contains(&'C') {
            commands.push(format!("cd {}", item.path))
        }
        if guide.contains(&'G') && item.is_git_hub_enabled() {
            commands.push(format!("cd {}", item.path));
            commands.push("gwb".to_string());
            commands.push("cd -".to_string());
        }
        if guide.contains(&'E') {
            commands.push(format!("open -n -a 'IntelliJ IDEA 2.app' --args {}", item.path));
        }
    }

    exit_ok(out, commands.join("; "))
}
