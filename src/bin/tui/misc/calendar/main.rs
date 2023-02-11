extern crate bins;
extern crate core;

use bins::libs::launcher::crossterm_launcher::launch;

mod day;
mod runner;
mod ui;

fn main() -> anyhow::Result<()> {
    launch(runner::run)
}
