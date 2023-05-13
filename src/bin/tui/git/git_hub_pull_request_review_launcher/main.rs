use bins::fuzzy::FuzzyBuilder;
use bins::io::command::run_command;

use crate::item::fetch_pull_requests;

mod item;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let items = fetch_pull_requests().await;

    let (items, guide) = FuzzyBuilder::tab(items, vec!["All", "Own", "Not-Reviewed", "Reviewed"])
        .guide(vec!["switch", "github"], vec![0])
        .build()
        .run()?;

    for item in items {
        if guide.contains(&'S') {
            checkout(item.get_number())?;
        }
        if guide.contains(&'G') {
            open(item.get_url())?;
        }
    }

    Ok(())
}

fn checkout(number: u64) -> anyhow::Result<()> {
    run_command("git fetch --prune")?;
    run_command("git reset .")?;
    run_command("git checkout .")?;
    run_command("git checkout -b reviewing-tmp")?;
    let _ = run_command("git branch -D reviewing");
    run_command(format!("git fetch origin pull/{number}/head:reviewing"))?;
    run_command("git checkout reviewing")?;
    run_command("git branch -D reviewing-tmp")
}

fn open(url: String) -> anyhow::Result<()> {
    run_command(format!("open {url}"))
}
