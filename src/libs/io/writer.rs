use serde::Serialize;
use std::fmt::Display;
use std::fs::{create_dir_all, remove_file, OpenOptions};
use std::io::stdout;
use std::io::Write;
use std::path::Path;
use std::process;

pub fn output_or_exit<T: Display>(value: T) -> anyhow::Result<()> {
    let r = writeln!(&mut stdout(), "{value}");
    if r.is_err() {
        process::exit(0);
    }
    Ok(())
}

pub fn write_serializable<T: ?Sized + Serialize>(path: &Path, t: &T) -> anyhow::Result<()> {
    let _ = remove_file(path);
    create_dir_all(path.parent().unwrap())?;
    let file = OpenOptions::new().write(true).create(true).open(path).unwrap();
    serde_yaml::to_writer(file, t)?;
    Ok(())
}
