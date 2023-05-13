extern crate bins;

use bins::io::command::print_command_out;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _ = &Opt::from_args();

    print_command_out("git add .").await
}
