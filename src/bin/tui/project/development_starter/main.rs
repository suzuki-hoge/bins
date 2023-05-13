use tui::layout::{Constraint, Direction};
use Constraint::Percentage;
use Direction::Horizontal;

use bins::fuzzy::FuzzyBuilder;
use bins::io::stdin::stdout;

use crate::item::parse_project_configs;

mod item;

fn main() -> anyhow::Result<()> {
    let items = parse_project_configs();

    let (items, guide) = FuzzyBuilder::pane(items, Horizontal, Percentage(30))
        .default_preview()
        .guide(vec!["cd", "edit", "git"], vec![0])
        .build()
        .run()?;

    let mut commands = vec![];

    for item in items {
        if guide.contains(&'C') {
            commands.push(format!("cd {}", item.work_dir))
        }
        if guide.contains(&'G') && item.is_git_hub_enabled() {
            commands.push(format!("cd {}", item.work_dir));
            commands.push("gwb".to_string());
            commands.push("cd -".to_string());
        }
        if guide.contains(&'E') {
            commands.push(format!("open -n -a 'IntelliJ IDEA.app' --args {}", item.work_dir));
        }
    }

    stdout(commands.join("; "))
}
