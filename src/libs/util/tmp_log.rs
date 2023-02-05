use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;

pub fn tmp_log<T: Debug>(t: T) {
    let mut file = OpenOptions::new().append(true).open("/tmp/bins.log").unwrap();
    file.write_all(format!("{t:?}\n").as_bytes()).unwrap();
    file.flush().unwrap();
}
