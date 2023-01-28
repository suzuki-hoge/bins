use itertools::Itertools;
use std::fs;
use std::path::Path;

#[derive(Eq, PartialEq, Debug)]
pub struct GitConfig {
    pub owner: String,
    pub repo: String,
}

pub fn get_git_config(dir_path: &Path) -> anyhow::Result<GitConfig> {
    let lines = fs::read_to_string(dir_path.join(".git").join("config"))?;
    let url_line = lines.split('\n').map(|line| line.trim()).filter(|line| line.starts_with("url")).collect_vec()[0];

    let parts = url_line.split('/').rev().collect_vec();
    let owner = parts[1].split(':').rev().collect_vec()[0].to_string();
    let repo = parts[0].replace(".git", "");

    Ok(GitConfig { owner, repo })
}

#[cfg(test)]
mod tests {
    use crate::libs::git::config::{get_git_config, GitConfig};
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use trim_margin::MarginTrimmable;

    fn setup_https(dir_path: &PathBuf) {
        let raw = r#"
            |[remote "origin"]
	        |	url = https://suzuki-hoge@github.com/suzuki-hoge/bins.git
        "#
        .trim()
        .trim_margin()
        .unwrap();

        let _ = fs::create_dir_all(dir_path);
        let _ = fs::create_dir_all(dir_path.join(".git"));
        let _ = File::create(dir_path.join(".git").join("config")).unwrap().write_all(raw.as_bytes());
    }

    fn setup_ssh(dir_path: &PathBuf) {
        let raw = r#"
            |[remote "origin"]
	        |	url = git@github.com:suzuki-hoge/bins.git
        "#
        .trim()
        .trim_margin()
        .unwrap();

        let _ = fs::create_dir_all(dir_path);
        let _ = fs::create_dir_all(dir_path.join(".git"));
        let _ = File::create(dir_path.join(".git").join("config")).unwrap().write_all(raw.as_bytes());
    }

    fn cleanup(dir_path: &PathBuf) {
        let _ = fs::remove_dir_all(dir_path);
    }

    #[test]
    fn found_https() {
        let dir_path = PathBuf::from("target/command-launcher/test-pj/github-https-found");

        setup_https(&dir_path);

        let act = get_git_config(&dir_path);
        let exp = GitConfig { owner: "suzuki-hoge".to_string(), repo: "bins".to_string() };

        assert_eq!(act.unwrap(), exp);

        cleanup(&dir_path);
    }

    #[test]
    fn found_ssh() {
        let dir_path = PathBuf::from("target/command-launcher/test-pj/github-ssh-found");

        setup_ssh(&dir_path);

        let act = get_git_config(&dir_path);
        let exp = GitConfig { owner: "suzuki-hoge".to_string(), repo: "bins".to_string() };

        assert_eq!(act.unwrap(), exp);

        cleanup(&dir_path);
    }

    #[test]
    fn notfound() {
        let dir_path = PathBuf::from("target/command-launcher/test-pj/github-found");

        let act = get_git_config(&dir_path);

        assert!(act.is_err());
    }
}
