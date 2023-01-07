use crossterm::tty::IsTty;
use std::io::{stdin, Error, ErrorKind, Result};

pub fn get_piped_stdin() -> Result<Vec<String>> {
    let stdin = stdin();

    if stdin.is_tty() {
        return Err(Error::new(ErrorKind::Other, "no piped stdin."));
    }

    let mut lines = vec![];
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).ok();
        if line.is_empty() {
            return Ok(lines);
        } else {
            lines.push(line.trim().to_string());
        }
    }
}
