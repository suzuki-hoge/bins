use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;

#[allow(dead_code)]
pub fn log<T: Debug>(t: T) {
    let mut file = OpenOptions::new().append(true).create(true).open("/tmp/bins.log").unwrap();
    file.write_all(format!("{t:?}\n").as_bytes()).unwrap();
    file.flush().unwrap();
}
