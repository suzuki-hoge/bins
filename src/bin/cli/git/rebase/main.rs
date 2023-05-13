extern crate bins;

use bins::git::branch::get_git_branch;
use bins::io::command::print_command_out;
use bins::io::stdin::stdout;
use question::{Answer, Question};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = Opt::from_args();

    let branch = get_git_branch()?;

    if let Some(base) = branch.base {
        let command = create_command(&base);
        match Question::new(&format!("{command} ? [y/n]")).confirm() {
            Answer::YES => print_command_out(command).await,
            _ => stdout("abort"),
        }
    } else {
        stdout("can't infer base branch")
    }
}

fn create_command(branch: &str) -> String {
    format!("git rebase {branch}")
}
