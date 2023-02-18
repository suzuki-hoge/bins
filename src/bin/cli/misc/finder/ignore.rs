
use std::path::{PathBuf};

use serde::Deserialize;

use bins::libs::io::reader::read_deserializable;

#[derive(Deserialize, Debug)]
pub struct Ignore {
    pub directories: Vec<String>,
}

pub fn get_ignores() -> anyhow::Result<Ignore> {
    let bins_dir = PathBuf::from(std::env::var("HOME")?);
    let yaml_path = bins_dir.join(".bins-finder-ignore.yaml");

    read_deserializable(&yaml_path)
}
