use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;

pub mod apps;
pub mod launcher;

pub fn log<T: Debug>(t: T) {
    let mut file = OpenOptions::new()
        .append(true)
        .open("/tmp/bins.log")
        .unwrap();
    file.write_all(format!("{:?}\n", t).as_bytes()).unwrap();
    file.flush().unwrap();
}
