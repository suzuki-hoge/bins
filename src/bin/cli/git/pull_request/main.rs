extern crate bins;

use std::env::current_dir;
use std::path::PathBuf;

use structopt::StructOpt;

use bins::libs::git::branch::{get_git_branch, GitBranch};
use bins::libs::git::config::get_git_config;
use bins::libs::io::writer::output_or_exit;
use bins::libs::process::command::run_command;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "destination", help = "destination branch ( infer base branch if missing )")]
    destination: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let git_config = get_git_config()?;
    let branch = get_branch()?;

    if let Some(base) = infer_base_branch(opt.destination, &branch) {
        let command = create_command(&git_config.owner, &git_config.repo, &base, &branch.current);
        run_command(command)
    } else {
        output_or_exit("can't infer base branch")
    }
}

fn get_branch() -> anyhow::Result<GitBranch> {
    let home = PathBuf::from(std::env::var("HOME")?);
    let dir_path = current_dir()?;

    get_git_branch(&home, &dir_path)
}

fn infer_base_branch(destination: Option<String>, branch: &GitBranch) -> Option<String> {
    match (destination, branch.base.clone()) {
        (Some(destination), _) => Some(destination),
        (None, Some(base)) => Some(base),
        (None, None) => None,
    }
}

fn create_command(owner: &str, repo: &str, base: &str, current: &str) -> String {
    format!("open https://github.com/{owner}/{repo}/compare/{base}...{current}")
}
