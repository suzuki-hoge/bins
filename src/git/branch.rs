use crate::git::branch_memo::get_branch_memos;
use crate::io::command::get_command_out_line;

#[derive(Eq, PartialEq, Debug)]
pub struct GitBranch {
    pub current: String,
    pub base: Option<String>,
}

pub fn get_git_branch() -> anyhow::Result<GitBranch> {
    let current = get_current()?;
    let base = get_base(&current)?;

    Ok(GitBranch { current, base })
}

fn get_current() -> anyhow::Result<String> {
    get_command_out_line("git rev-parse --abbrev-ref head")
}

fn get_base(current: &str) -> anyhow::Result<Option<String>> {
    get_branch_memos().map(|memos| memos.iter().find(|memo| memo.current == current).map(|memo| memo.base.to_string()))
}
