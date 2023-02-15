use std::fs::read_to_string;
use std::path::PathBuf;

pub fn get_git_token() -> anyhow::Result<String> {
    let bins_dir = PathBuf::from(std::env::var("HOME")?);
    let token = read_to_string(bins_dir.join(".bins-git-token"))?;
    Ok(token.trim().to_string())
}
