use crossterm::tty::IsTty;
use itertools::Itertools;
use std::io::{stdin, Error, ErrorKind, Result};
use std::process::Command;

pub fn get_piped_stdin_or_dummy() -> Result<Vec<String>> {
    if stdin().is_tty() {
        dummy()
    } else {
        get_piped_stdin()
    }
}

fn dummy() -> Result<Vec<String>> {
    let o = Command::new("ps").args(["aux"]).output()?;
    Ok(String::from_utf8_lossy(&o.stdout).split('\n').into_iter().map(|s| s.to_string()).collect_vec())
}

fn get_piped_stdin() -> Result<Vec<String>> {
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
