extern crate bins;

use structopt::StructOpt;

use bins::libs::process::command::run_command;

#[derive(StructOpt)]
struct Opt {}

fn main() -> anyhow::Result<()> {
    let _ = &Opt::from_args();

    run_command("git reset .")?;
    run_command("git checkout .")?;
    run_command("git clean -d -f")
}
