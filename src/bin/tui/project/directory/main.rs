use structopt::StructOpt;
use tui::layout::{Constraint, Direction};
use Constraint::Percentage;
use Direction::Horizontal;

use bins::fuzzy::FuzzyBuilder;
use bins::io::stdin::stdout;

use crate::item::{get_config_path, parse_items};

mod item;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "e", long = "--edit", help = "edit config")]
    edit: bool,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    match opt.edit {
        true => edit(),
        false => boot(),
    }
}

fn edit() -> anyhow::Result<()> {
    stdout(format!("vi {}", get_config_path().display()))
}

fn boot() -> anyhow::Result<()> {
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

    stdout(commands.join("; "))
}
