use structopt::StructOpt;

use bins::libs::process::command::{print_command_out, run_command};

#[derive(StructOpt)]
struct Opt {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = &Opt::from_args();

    run_command("git config --global color.status always")?;

    print_command_out("git status --short --branch").await?;

    run_command("git config --global color.status auto")
}
