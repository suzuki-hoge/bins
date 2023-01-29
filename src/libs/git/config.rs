use itertools::Itertools;
use std::process::Command;

#[derive(Eq, PartialEq, Debug)]
pub struct GitConfig {
    pub owner: String,
    pub repo: String,
}

pub fn get_git_config() -> anyhow::Result<GitConfig> {
    let o = Command::new("git").args(["config", "--get", "remote.origin.url"]).output()?;
    let url = String::from_utf8_lossy(&o.stdout).trim().to_string();

    let parts = url.split('/').rev().collect_vec();
    let owner = parts[1].split(':').rev().collect_vec()[0].to_string();
    let repo = parts[0].replace(".git", "");

    Ok(GitConfig { owner, repo })
}
