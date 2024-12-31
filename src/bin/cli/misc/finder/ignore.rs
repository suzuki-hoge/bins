use bins::io::file::{get_bins_dir, read_deserializable};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Ignore {
    pub directories: Vec<String>,
}

pub fn get_ignores() -> anyhow::Result<Ignore> {
    let yaml_path = get_bins_dir().join("finder-ignore.yaml");

    read_deserializable(yaml_path)
}
