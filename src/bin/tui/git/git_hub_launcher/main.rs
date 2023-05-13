use bins::fuzzy::FuzzyBuilder;
use bins::git::branch::get_git_branch;
use bins::io::command::run_command;

use crate::item::gather_urls;

mod item;

fn main() -> anyhow::Result<()> {
    let git_branch = get_git_branch()?;

    let items = gather_urls(&git_branch);
    let tabs = if let Some(base) = git_branch.base {
        vec![String::from("github"), git_branch.current, base]
    } else {
        vec![String::from("github"), git_branch.current]
    };

    let (items, _) = FuzzyBuilder::tab(items, tabs).build().run()?;

    for item in items {
        open(item.url)?
    }

    Ok(())
}

fn open(url: String) -> anyhow::Result<()> {
    run_command(format!("open {url}"))
}
