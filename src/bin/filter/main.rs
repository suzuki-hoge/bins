extern crate bins;

use itertools::Itertools;

use bins::libs::launcher::crossterm_launcher::crossterm_launcher;

mod app;
mod runner;
mod ui;

fn main() {
    match crossterm_launcher(|terminal| runner::run(terminal, vec!["abc".to_string(), "def".to_string()])) {
        Ok(values) => println!("{}", values.iter().join("\n")),
        Err(e) => println!("process error: {}", e),
    }
}
