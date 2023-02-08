extern crate bins;

use bins::libs::io::reader::get_piped_stdin_or_dummy;
use itertools::Itertools;

use bins::libs::launcher::crossterm_launcher::launch;

mod runner;
mod ui;

use bins::libs::io::writer::stdout;

fn main() -> anyhow::Result<()> {
    match launch(|terminal| runner::run(terminal, get_piped_stdin_or_dummy()?)) {
        Ok(items) => stdout(items.iter().join("\n")),
        Err(e) => stdout(format!("echo {e}")),
    }
}
