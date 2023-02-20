extern crate bins;

use bins::libs::io::writer::{stderr, stdout};
use bins::libs::launcher::crossterm_launcher::launch;

use crate::project_config::ProjectItem;
use crate::runner::Actions;

mod project_config;
mod runner;
mod ui;

fn main() -> anyhow::Result<()> {
    match launch(runner::run) {
        Ok((items, _)) if items.is_empty() => Ok(()),
        Ok((items, actions)) => eval(items[0].clone(), actions),
        Err(e) => stderr(e),
    }
}

fn eval(item: ProjectItem, action: Actions) -> anyhow::Result<()> {
    let mut commands = vec![];
    if action.cd {
        commands.push(format!("cd {}", item.origin.work_dir_path));
    }
    if action.edit {
        commands.push(format!("open -n -a 'IntelliJ IDEA.app' --args {}", item.origin.work_dir_path));
    }
    if action.github && item.origin.git_exists {
        commands.push(format!("cd {}", item.origin.work_dir_path));
        commands.push("gwb".to_string());
    }
    if action.up && item.origin.up_exists {
        commands.push(format!("cd {}", item.origin.work_dir_path));
        commands.push("bb u".to_string());
    }
    stdout(commands.join("; "))
}
