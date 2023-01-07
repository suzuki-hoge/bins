extern crate bins;

use bins::libs::io::input::get_piped_stdin;
use itertools::Itertools;
use std::fmt::Display;
use std::io::stdout;

use bins::libs::launcher::crossterm_launcher::crossterm_launcher;

mod app;
mod runner;
mod ui;

use std::io::Write;
use std::process;

fn main() {
    match crossterm_launcher(|terminal| runner::run(terminal, get_piped_stdin()?)) {
        Ok(values) => out(values.iter().join("\n")),
        Err(e) => out(e),
    }
}

fn out<T: Display>(value: T) {
    let r = writeln!(&mut stdout(), "{}", value);
    if r.is_err() {
        process::exit(0);
    }
}
