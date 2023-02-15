extern crate bins;

use bins::libs::git::config::{get_git_config, GitConfig};
use bins::libs::git::username::get_git_username;
use bins::libs::io::writer::stdout;
use bins::libs::launcher::crossterm_launcher::launch;
use bins::libs::process::command::run_command;

use structopt::StructOpt;

use crate::http::fetch_pull_requests;
use crate::pull_request::PullRequest;
use crate::runner::Actions;

mod http;
mod pull_request;
mod runner;
mod ui;

#[derive(StructOpt)]
struct Opt {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = Opt::from_args();

    let git_config = get_git_config()?;
    let pull_requests = fetch_pull_requests(&git_config).await?;
    let username = get_git_username()?;

    match launch(|terminal| runner::run(terminal, pull_requests.clone(), &username)) {
        Ok((items, _)) if items.is_empty() => Ok(()),
        Ok((items, actions)) => eval(items[0].clone(), actions, &git_config),
        Err(e) => stdout(e),
    }
}

fn eval(item: PullRequest, action: Actions, git_config: &GitConfig) -> anyhow::Result<()> {
    if action.switch {
        run_command("git fetch --prune")?;
        run_command("git reset .")?;
        run_command("git checkout .")?;
        run_command("git checkout -b reviewing-tmp")?;
        let _ = run_command("git branch -D reviewing");
        run_command(format!("git fetch origin pull/{}/head:reviewing", item.get_number()))?;
        run_command("git checkout reviewing")?;
        run_command("git branch -D reviewing-tmp")?;
    }
    if action.git_hub {
        run_command(format!("open {}", item.get_url(git_config)))?;
    }
    Ok(())
}
