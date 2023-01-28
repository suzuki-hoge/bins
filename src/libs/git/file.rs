use itertools::Itertools;
use std::process::Command;

pub fn get_git_paths() -> Vec<String> {
    match Command::new("git").args(["ls-files"]).output() {
        Ok(o) => String::from_utf8_lossy(&o.stdout)
            .split('\n')
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string())
            .collect_vec(),
        Err(_) => vec![],
    }
}
