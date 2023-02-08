extern crate bins;

use structopt::StructOpt;

use bins::libs::process::command::print_command_out;

#[derive(StructOpt)]
struct Opt {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = &Opt::from_args();

    print_command_out("git add .").await
}
