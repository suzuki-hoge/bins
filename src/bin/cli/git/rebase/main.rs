extern crate bins;

use question::{Answer, Question};
use std::env::current_dir;
use std::path::PathBuf;

use bins::libs::git::branch::{get_git_branch, GitBranch};
use bins::libs::io::writer::output_or_exit;
use bins::libs::process::command::print_command_out;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {}

fn main() -> anyhow::Result<()> {
    let _ = Opt::from_args();

    let branch = get_branch()?;

    if let Some(base) = branch.base {
        let command = create_command(&base);
        match Question::new(&format!("{command} ? [y/n]")).confirm() {
            Answer::YES => print_command_out(command),
            _ => output_or_exit("abort"),
        }
    } else {
        output_or_exit("can't infer base branch")
    }
}

fn get_branch() -> anyhow::Result<GitBranch> {
    let home = PathBuf::from(std::env::var("HOME")?);
    let dir_path = current_dir()?;

    get_git_branch(&home, &dir_path)
}

fn create_command(branch: &str) -> String {
    format!("git rebase {branch}")
}
