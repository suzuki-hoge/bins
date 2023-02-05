extern crate bins;

use structopt::StructOpt;

use bins::libs::git::branch::get_git_branch;
use bins::libs::git::config::get_git_config;
use bins::libs::io::writer::output_or_exit;
use bins::libs::process::command::run_command;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "destination", help = "destination branch ( infer base branch if missing )")]
    destination: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let git_config = get_git_config()?;
    let branch = get_git_branch()?;

    if let Some(base) = infer_base_branch(opt.destination, branch.base.as_deref()) {
        let command = create_command(&git_config.owner, &git_config.repo, &base, &branch.current);
        run_command(command)
    } else {
        output_or_exit("can't infer base branch")
    }
}

fn infer_base_branch(destination: Option<String>, base: Option<&str>) -> Option<String> {
    match (destination, base) {
        (Some(destination), _) => Some(destination),
        (None, Some(base)) => Some(base.to_string()),
        (None, None) => None,
    }
}

fn create_command(owner: &str, repo: &str, base: &str, current: &str) -> String {
    format!("open https://github.com/{owner}/{repo}/compare/{base}...{current}")
}
