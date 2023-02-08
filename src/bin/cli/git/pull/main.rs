extern crate bins;

use structopt::StructOpt;

use bins::libs::git::branch::get_git_branch;
use bins::libs::process::command::print_command_out;

#[derive(StructOpt)]
struct Opt {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = Opt::from_args();

    let branch = get_git_branch()?;

    let command = create_command(&branch.current);
    print_command_out(command).await
}

fn create_command(branch: &str) -> String {
    format!("git pull origin {branch}")
}
