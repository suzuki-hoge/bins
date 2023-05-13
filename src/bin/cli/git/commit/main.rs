extern crate bins;

use structopt::StructOpt;

use bins::io::command::print_command_out;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "message")]
    message: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let command = create_command(&opt.message);
    print_command_out(command).await
}

fn create_command(message: &str) -> String {
    format!("git commit --message '{message}'")
}
