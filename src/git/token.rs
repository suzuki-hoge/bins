use crate::io::file::get_bins_dir;
use std::fs::read_to_string;

pub fn get_git_token() -> String {
    let token = read_to_string(get_bins_dir().join("git-token")).unwrap();
    token.trim().to_string()
}
