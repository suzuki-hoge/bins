extern crate bins;

use bins::libs::git::branch::{get_git_branch, GitBranch};
use bins::libs::git::config::{get_git_config, GitConfig};
use bins::libs::git::file::get_git_paths;
use itertools::Itertools;
use std::env::current_dir;

use bins::libs::io::writer::output_or_exit;

fn main() -> anyhow::Result<()> {
    let dir_path = current_dir()?;

    let git_config = get_git_config(&dir_path)?;
    let git_branch = get_git_branch()?;

    let lines = vec![
        get_pulls(&git_config),
        get_issues(&git_config),
        get_tree(&git_config, &git_branch),
        get_commits(&git_config, &git_branch),
        get_compare(&git_config, &git_branch),
        get_wiki(&git_config),
        get_find(&git_config, &git_branch),
        get_blob(&git_config, &git_branch),
    ]
    .into_iter()
    .flatten()
    .collect_vec();
    output_or_exit(lines.join("\n"))
}

const HOST: &str = "https://github.com";

fn get_pulls(config: &GitConfig) -> Vec<String> {
    vec![
        format!("{}/{}/{}/pulls", HOST, config.owner, config.repo),
        format!("{}/{}/{}/pulls/@me", HOST, config.owner, config.repo),
        format!("{}/{}/{}/pulls?q=is:open+is:pr+-reviewed-by:@me+reviewed-by:@me", HOST, config.owner, config.repo),
    ]
}

fn get_issues(config: &GitConfig) -> Vec<String> {
    vec![format!("{}/{}/{}/issues", HOST, config.owner, config.repo)]
}

fn get_tree(config: &GitConfig, branch: &GitBranch) -> Vec<String> {
    branch
        .get_all()
        .iter()
        .map(|branch| format!("{}/{}/{}/tree/{}", HOST, config.owner, config.repo, branch))
        .collect_vec()
}

fn get_commits(config: &GitConfig, branch: &GitBranch) -> Vec<String> {
    branch
        .get_all()
        .iter()
        .map(|branch| format!("{}/{}/{}/commits/{}", HOST, config.owner, config.repo, branch))
        .collect_vec()
}

fn get_compare(config: &GitConfig, branch: &GitBranch) -> Vec<String> {
    vec![format!("{}/{}/{}/commits/{}...{}", HOST, config.owner, config.repo, branch.base, branch.current)]
}

fn get_wiki(config: &GitConfig) -> Vec<String> {
    vec![format!("{}/{}/{}/wiki", HOST, config.owner, config.repo)]
}

fn get_find(config: &GitConfig, branch: &GitBranch) -> Vec<String> {
    branch
        .get_all()
        .iter()
        .map(|branch| format!("{}/{}/{}/find/{}", HOST, config.owner, config.repo, branch))
        .collect_vec()
}

fn get_blob(config: &GitConfig, branch: &GitBranch) -> Vec<String> {
    let paths = get_git_paths();
    branch
        .get_all()
        .iter()
        .flat_map(|branch| {
            paths.iter().map(move |path| format!("{}/{}/{}/blob/{}/{}", HOST, config.owner, config.repo, branch, path))
        })
        .collect_vec()
}
