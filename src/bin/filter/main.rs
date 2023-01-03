extern crate bins;

use itertools::Itertools;

use bins::libs::launcher::crossterm_launcher::crossterm_launcher;

mod app;
mod runner;
mod ui;

fn main() {
    match crossterm_launcher(runner::run) {
        Ok(vs) => println!("{}", vs.iter().join("\n")),
        Err(e) => println!("{}", e),
    }
}
