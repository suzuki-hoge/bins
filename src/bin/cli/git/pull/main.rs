extern crate bins;

use bins::git::branch::get_git_branch;
use bins::io::command::print_command_out;
use structopt::StructOpt;

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
