extern crate bins;

mod app;
mod runner;
mod ui;

fn main() {
    match bins::launcher::crossterm_launcher(runner::run) {
        Ok(v) => println!("{}", v),
        Err(e) => println!("{}", e),
    }
}
