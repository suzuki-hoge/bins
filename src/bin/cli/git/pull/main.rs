extern crate bins;

use std::env::current_dir;
use std::path::PathBuf;

use structopt::StructOpt;

use bins::libs::git::branch::{get_git_branch, GitBranch};
use bins::libs::process::command::print_command_out;

#[derive(StructOpt)]
struct Opt {}

fn main() -> anyhow::Result<()> {
    let _ = Opt::from_args();

    let branch = get_branch()?;

    let command = create_command(&branch.current);
    print_command_out(command)
}

fn get_branch() -> anyhow::Result<GitBranch> {
    let home = PathBuf::from(std::env::var("HOME")?);
    let dir_path = current_dir()?;

    get_git_branch(&home, &dir_path)
}

fn create_command(branch: &str) -> String {
    format!("git pull origin {branch}")
}
