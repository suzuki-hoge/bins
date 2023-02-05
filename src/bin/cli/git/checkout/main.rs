extern crate bins;

use structopt::StructOpt;

use bins::libs::git::branch::get_git_branch;
use bins::libs::git::branch_memo::add_branch_memo;
use bins::libs::process::command::print_command_out;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "b", long = "--branch", help = "create and checkout a new branch")]
    branch: bool,
    #[structopt(short = "f", long = "--feature", help = "create sub branch under feature branch")]
    feature: bool,
    #[structopt(short = "o", long = "--origin", help = "checkout from remote origin")]
    origin: bool,
    #[structopt(short = "p", long = "--prefix", help = "add branch prefix")]
    prefix: Option<String>,
    #[structopt(name = "target", help = "branch name or file name")]
    target: String,
}

fn main() -> anyhow::Result<()> {
    let opt = &Opt::from_args();

    let branch = get_git_branch()?;
    let target = create_target(opt.prefix.as_deref(), opt.feature, &opt.target);
    let command = create_command(opt.branch, opt.origin, &target);

    add_branch_memo(branch.current, target)?;

    print_command_out(command)
}

fn create_target(prefix: Option<&str>, feature: bool, target: &str) -> String {
    match (prefix, feature) {
        (Some(prefix), true) => format!("{prefix}/feature/{target}"),
        (Some(prefix), false) => format!("{prefix}/{target}"),
        (None, true) => format!("feature/{target}"),
        (None, false) => target.to_string(),
    }
}

fn create_command(branch: bool, origin: bool, target: &str) -> String {
    match (branch, origin) {
        (true, true) => format!("git checkout -b {target} origin/{target}"),
        (true, false) => format!("git checkout -b {target}"),
        (false, true) => format!("git checkout {target} origin/{target}"),
        (false, false) => format!("git checkout {target}"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{create_command, create_target};

    #[test]
    fn test_create_target() {
        assert_eq!(create_target(Some("auth"), true, "issue-42"), "auth/feature/issue-42".to_string());
        assert_eq!(create_target(None, true, "issue-42"), "feature/issue-42".to_string());
        assert_eq!(create_target(Some("auth"), false, "issue-42"), "auth/issue-42".to_string());
        assert_eq!(create_target(None, false, "issue-42"), "issue-42".to_string());
    }

    #[test]
    fn test_create_command() {
        assert_eq!(create_command(true, true, "issue-42"), "git checkout -b issue-42 origin/issue-42".to_string());
        assert_eq!(create_command(true, false, "issue-42"), "git checkout -b issue-42".to_string());
        assert_eq!(create_command(false, true, "issue-42"), "git checkout issue-42 origin/issue-42".to_string());
        assert_eq!(create_command(false, false, "issue-42"), "git checkout issue-42".to_string());
    }
}
