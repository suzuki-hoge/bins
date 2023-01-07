extern crate bins;

use bins::libs::io::input::get_piped_stdin;
use itertools::Itertools;

use bins::libs::launcher::crossterm_launcher::crossterm_launcher;

mod app;
mod runner;
mod ui;

fn main() {
    match crossterm_launcher(|terminal| runner::run(terminal, get_piped_stdin()?)) {
        Ok(values) => println!("{}", values.iter().join("\n")),
        Err(e) => println!("{}", e),
    }
}
