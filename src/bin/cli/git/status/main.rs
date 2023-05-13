use bins::io::command::{print_command_out, run_command};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = &Opt::from_args();

    run_command("git config --global color.status always")?;

    print_command_out("git status --short --branch").await?;

    run_command("git config --global color.status auto")
}
