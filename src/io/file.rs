use anyhow::Context;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fs::{create_dir_all, remove_file, File, OpenOptions};
use std::io::BufReader;
use std::path::Path;

pub fn read_deserializable<P: AsRef<Path>, T: DeserializeOwned>(path: P) -> anyhow::Result<T> {
    let file = File::open(path).context("no such file")?;
    let reader = BufReader::new(file);
    serde_yaml::from_reader(reader).context("yaml parse error")
}

pub fn write_serializable<P: AsRef<Path>, T: ?Sized + Serialize>(path: P, t: &T) -> anyhow::Result<()> {
    let path = path.as_ref();
    let _ = remove_file(path);
    create_dir_all(path.parent().unwrap())?;
    let file = OpenOptions::new().write(true).create(true).open(path).unwrap();
    serde_yaml::to_writer(file, t)?;
    Ok(())
}

pub fn delete_file<P: AsRef<Path>>(path: P) -> anyhow::Result<()> {
    let _ = remove_file(path);
    Ok(())
}
