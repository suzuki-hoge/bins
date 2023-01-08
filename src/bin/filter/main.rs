extern crate bins;

use bins::libs::io::reader::get_piped_stdin_or_dummy;
use itertools::Itertools;

use bins::libs::launcher::crossterm_launcher::crossterm_launcher;

mod app;
mod runner;
mod ui;

use bins::libs::io::writer::output_or_exit;

fn main() {
    match crossterm_launcher(|terminal| runner::run(terminal, get_piped_stdin_or_dummy()?)) {
        Ok(values) => output_or_exit(values.iter().join("\n")),
        Err(e) => output_or_exit(e),
    }
}
