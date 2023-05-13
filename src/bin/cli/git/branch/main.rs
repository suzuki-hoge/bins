extern crate bins;

use bins::io::command::{print_command_out, run_command};
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "a", long = "--all", help = "list both remote-tracking and local branches")]
    all: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = &Opt::from_args();

    run_command("git config --global color.branch always")?;

    let command = create_command(opt.all);
    print_command_out(command).await?;

    run_command("git config --global color.branch auto")
}

fn create_command(all: bool) -> &'static str {
    match all {
        true => "git branch --all",
        false => "git branch",
    }
}
