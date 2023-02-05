use crate::libs::process::command::get_command_out_line;
use itertools::Itertools;

#[derive(Eq, PartialEq, Debug)]
pub struct GitConfig {
    pub owner: String,
    pub repo: String,
}

pub fn get_git_config() -> anyhow::Result<GitConfig> {
    let line = get_command_out_line("git config --get remote.origin.url")?;

    let sp = line.split('/').rev().collect_vec();

    let owner = sp[1].split(':').rev().collect_vec()[0].to_string();
    let repo = sp[0].replace(".git", "");

    Ok(GitConfig { owner, repo })
}
