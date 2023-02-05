extern crate bins;

use structopt::StructOpt;

use bins::libs::process::command::print_command_out;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "message")]
    message: String,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let command = create_command(&opt.message);
    print_command_out(command)
}

fn create_command(message: &str) -> String {
    format!("git commit --message '{message}'")
}
