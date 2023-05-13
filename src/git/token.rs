use std::fs::read_to_string;
use std::path::PathBuf;

pub fn get_git_token() -> String {
    let home_dir = PathBuf::from(std::env::var("HOME").unwrap());
    let token = read_to_string(home_dir.join(".bins-git-token")).unwrap();
    token.trim().to_string()
}
