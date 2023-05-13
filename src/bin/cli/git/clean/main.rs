extern crate bins;

use bins::io::command::run_command;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {}

fn main() -> anyhow::Result<()> {
    let _ = &Opt::from_args();

    run_command("git reset .")?;
    run_command("git checkout .")?;
    run_command("git clean -d -f")
}
