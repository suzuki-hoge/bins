extern crate bins;

use itertools::Itertools;

mod app;
mod runner;
mod ui;

fn main() {
    match bins::launcher::crossterm_launcher(runner::run) {
        Ok(vs) => println!("{}", vs.iter().join("\n")),
        Err(e) => println!("{}", e),
    }
}
