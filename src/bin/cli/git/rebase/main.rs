extern crate bins;

use question::{Answer, Question};
use structopt::StructOpt;

use bins::libs::git::branch::get_git_branch;
use bins::libs::io::writer::stdout;
use bins::libs::process::command::print_command_out;

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
