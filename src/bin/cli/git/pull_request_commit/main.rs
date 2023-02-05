extern crate bins;

use itertools::Itertools;
use structopt::StructOpt;

use bins::libs::git::config::get_git_config;
use bins::libs::process::command::{get_command_out, run_command};

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "hash", help = "commit hash for search")]
    hash: String,
    #[structopt(name = "base", default_value = "develop", help = "base branch")]
    base: String,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let git_config = get_git_config()?;

    let command = create_log_command(&opt.hash, &opt.base);
    let logs = get_command_out(command)?;

    let number = &(logs[0].split(' ').collect_vec()[4])[1..];

    let command = create_open_command(&git_config.owner, &git_config.repo, number);
    run_command(command)
}

fn create_log_command(hash: &str, base: &str) -> String {
    format!("git log --merges --oneline --reverse --ancestry-path {hash}...{base}")
}

fn create_open_command(owner: &str, repo: &str, number: &str) -> String {
    format!("open https://github.com/{owner}/{repo}/pull/{number}")
}
