extern crate bins;

use bins::git::branch::get_git_branch;
use bins::io::command::print_command_out;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "f", long = "--force", help = "force update")]
    force: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let branch = get_git_branch()?;

    let command = create_command(opt.force, &branch.current);
    print_command_out(command).await
}

fn create_command(force: bool, branch: &str) -> String {
    match force {
        true => format!("git push -f origin {branch}"),
        false => format!("git push origin {branch}"),
    }
}
