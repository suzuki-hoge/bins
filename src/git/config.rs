use std::env::current_dir;
use std::fs::read_to_string;
use std::path::Path;

use itertools::Itertools;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct GitConfig {
    pub owner: String,
    pub repo: String,
}

pub fn get_current_git_config() -> Option<GitConfig> {
    let work_dir = current_dir().unwrap();

    get_git_config(work_dir)
}

pub fn get_git_config<P: AsRef<Path>>(p: P) -> Option<GitConfig> {
    match read_to_string(p.as_ref().join(".git").join("config")).map(|lines| {
        lines
            .split('\n')
            .filter(|line| line.contains("url = "))
            .map(|line| line.split('=').collect_vec()[1].trim().to_string())
            .join("")
    }) {
        Ok(line) if line.is_empty() => None,
        Ok(line) => {
            let sp = line.split('/').rev().collect_vec();

            let owner = sp[1].split(':').rev().collect_vec()[0].to_string();
            let repo = sp[0].replace(".git", "");

            Some(GitConfig { owner, repo })
        }
        Err(_) => None,
    }
}

#[cfg(test)]
mod tests {
    use crate::git::config::{get_git_config, GitConfig};
    use crate::io::command::get_command_out_line;

    #[test]
    fn found() {
        let bins = get_command_out_line("git rev-parse --show-toplevel").unwrap();
        let act = get_git_config(bins);

        assert_eq!(act, Some(GitConfig { owner: String::from("suzuki-hoge"), repo: String::from("bins") }))
    }

    #[test]
    fn notfound() {
        let act = get_git_config("/path/to/foo");

        assert_eq!(act, None)
    }
}
