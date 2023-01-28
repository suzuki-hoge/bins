use itertools::Itertools;
use regex::Regex;
use std::process::Command;

#[derive(Eq, PartialEq, Debug)]
pub struct GitBranch {
    pub current: String,
    pub base: String,
    other_locals: Vec<String>,
}

impl GitBranch {
    pub fn get_all(&self) -> Vec<String> {
        vec![
            vec![self.current.clone(), self.base.clone()].into_iter().unique().collect_vec(),
            self.other_locals.clone(),
        ]
        .into_iter()
        .flatten()
        .collect_vec()
    }
}

pub fn get_git_branch() -> anyhow::Result<GitBranch> {
    Ok(GitBranch { current: get_current()?, base: get_base()?, other_locals: get_locals()? })
}

fn get_current() -> anyhow::Result<String> {
    let o = Command::new("git").args(["rev-parse", "--abbrev-ref", "head"]).output()?;
    Ok(String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn get_base() -> anyhow::Result<String> {
    let o = Command::new("git").args(["show-branch"]).output()?;
    parse_base(String::from_utf8_lossy(&o.stdout).to_string())
}

fn parse_base(output: String) -> anyhow::Result<String> {
    let current = get_current()?;

    let sep = Regex::new(r"\n-+\n")?;
    let lines = sep.split(&output).into_iter().collect_vec()[1]
        .split('\n')
        .into_iter()
        .filter(|line| line.contains('*'))
        .filter(|line| !line.contains(&current))
        .collect_vec();

    if lines.is_empty() {
        Ok(current)
    } else {
        Ok(lines[0].split('[').collect_vec()[1].split(']').collect_vec()[0].to_string())
    }
}

fn get_locals() -> anyhow::Result<Vec<String>> {
    let current = get_current()?;
    let base = get_base()?;

    let o = Command::new("git").args(["branch"]).output()?;
    Ok(String::from_utf8_lossy(&o.stdout)
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s[2..].to_string())
        .filter(|s| s != &current || s != &base)
        .collect_vec())
}

#[cfg(test)]
mod tests {
    use crate::libs::git::branch::parse_base;
    use trim_margin::MarginTrimmable;

    #[test]
    fn found_base() {
        let raw = "
            |! [dev] 2
            | * [feat] 3
            |  ! [master] 1
            |---
            | *  [feat] 3
            |+*  [dev] 2
            |+*+ [master] 1
        "
        .trim()
        .trim_margin()
        .unwrap();

        let act = parse_base(raw);
        assert_eq!(act.unwrap(), "feat".to_string())
    }

    #[test]
    fn on_base() {
        let raw = "
            |! [dev] 2
            | * [master] 1
            |--
            |+  [dev] 2
            |+* [master] 1
        "
        .trim()
        .trim_margin()
        .unwrap();

        let act = parse_base(raw);
        assert_eq!(act.unwrap(), "master".to_string())
    }
}
