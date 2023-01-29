use anyhow::Context;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::process::Command;

#[derive(Eq, PartialEq, Debug)]
pub struct GitBranch {
    pub current: String,
    base: Option<String>,
    other_locals: Vec<String>,
}

impl GitBranch {
    pub fn get_all(&self) -> Vec<String> {
        let mut result = vec![self.current.clone()];
        self.base.iter().for_each(|base| result.push(base.clone()));
        result.append(&mut self.other_locals.clone());

        result
    }

    pub fn get_compare(&self) -> Option<(String, String)> {
        self.base.clone().map(|base| (base, self.current.clone()))
    }
}

#[derive(Serialize, Deserialize, Eq, PartialEq, Ord, PartialOrd, Clone, Debug)]
struct Memo {
    base: String,
    current: String,
}

pub fn get_git_branch(yaml_dir_path: &Path, current_dir_path: &Path) -> anyhow::Result<GitBranch> {
    let current = get_current()?;
    let base = get_base(yaml_dir_path, current_dir_path, &current)?;
    let other_locals = get_other_locals(&current, base.as_ref())?;
    Ok(GitBranch { current, base, other_locals })
}

fn get_current() -> anyhow::Result<String> {
    let o = Command::new("git").args(["rev-parse", "--abbrev-ref", "head"]).output()?;
    Ok(String::from_utf8_lossy(&o.stdout).trim().to_string())
}

fn get_base(yaml_dir_path: &Path, current_dir_path: &Path, current: &str) -> anyhow::Result<Option<String>> {
    let current_dir_path = current_dir_path.display().to_string().replace('/', ".");
    let path = yaml_dir_path.join(".bins-branch").join(format!("{}.yaml", current_dir_path)).display().to_string();

    match read_file(&path) {
        Ok(branch_memos) => {
            if let Some(memo) = branch_memos.into_iter().find(|memo| memo.current == current) {
                Ok(Some(memo.base))
            } else {
                Ok(None)
            }
        }
        Err(_) => Ok(None),
    }
}

fn read_file(path: &String) -> anyhow::Result<Vec<Memo>> {
    let file = File::open(path).context("no such file")?;
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader).context("yaml parse error")
}

fn get_other_locals(current: &str, base: Option<&String>) -> anyhow::Result<Vec<String>> {
    let o = Command::new("git").args(["branch"]).output()?;
    Ok(String::from_utf8_lossy(&o.stdout)
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|s| s[2..].to_string())
        .filter(|s| s != current && s != base.unwrap_or(&"".to_string()))
        .collect_vec())
}

#[cfg(test)]
mod tests {
    use crate::libs::git::branch::get_base;
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use trim_margin::MarginTrimmable;

    fn setup(dir_path: &PathBuf) {
        let raw = "
            |- { base: develop, current: feat }
            |- { base: feat, current: sub-feat }
        "
        .trim()
        .trim_margin()
        .unwrap();

        let _ = fs::create_dir_all(dir_path);
        let _ = fs::create_dir_all(dir_path.join(".bins-branch"));
        let _ = File::create(dir_path.join(".bins-branch").join(".path.front.yaml")).unwrap().write_all(raw.as_bytes());
    }

    fn cleanup(dir_path: &PathBuf) {
        let _ = fs::remove_dir_all(dir_path);
    }

    #[test]
    fn found() {
        let yaml_dir_path = PathBuf::from("target/git/test-pj/found");

        setup(&yaml_dir_path);

        let current_dir_path = PathBuf::from("/path/front");

        let act = get_base(&yaml_dir_path, &current_dir_path, "feat");
        assert_eq!(act.unwrap(), Some("develop".to_string()));

        cleanup(&yaml_dir_path);
    }

    #[test]
    fn memo_line_notfound() {
        let yaml_dir_path = PathBuf::from("target/git/test-pj/memo-line-notfound");

        setup(&yaml_dir_path);

        let current_dir_path = PathBuf::from("/path/front");

        let act = get_base(&yaml_dir_path, &current_dir_path, "hotfix");
        assert_eq!(act.unwrap(), None);

        cleanup(&yaml_dir_path);
    }

    #[test]
    fn memo_file_notfound() {
        let yaml_dir_path = PathBuf::from("target/git/test-pj/memo-file-notfound");

        let current_dir_path = PathBuf::from("/path/front");

        let act = get_base(&yaml_dir_path, &current_dir_path, "feat");
        assert_eq!(act.unwrap(), None)
    }
}
