use serde::Serialize;
use std::fmt::Display;
use std::fs::{create_dir_all, remove_file, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::process;

pub fn stdout<T: Display>(value: T) -> anyhow::Result<()> {
    let r = writeln!(&mut std::io::stdout(), "{value}");
    if r.is_err() {
        process::exit(0);
    }
    Ok(())
}

pub fn stderr<T: Display>(value: T) -> anyhow::Result<()> {
    eprintln!("{value}");
    Ok(())
}

pub fn write_serializable<T: ?Sized + Serialize>(path: &Path, t: &T) -> anyhow::Result<()> {
    let _ = remove_file(path);
    create_dir_all(path.parent().unwrap())?;
    let file = OpenOptions::new().write(true).create(true).open(path).unwrap();
    serde_yaml::to_writer(file, t)?;
    Ok(())
}
