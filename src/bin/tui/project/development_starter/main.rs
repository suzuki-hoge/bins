extern crate bins;

// use structopt::StructOpt;

use bins::libs::io::writer::{stderr, stdout};
use bins::libs::launcher::crossterm_launcher::launch;


mod project_config;
mod runner;
mod ui;

// #[derive(StructOpt)]
// struct Opt {
//     #[structopt(short = "g", long = "--generate", help = "generate empty config")]
//     generate: bool,
//
//     #[structopt(name = "command_name", help = "run specified command instantly")]
//     name: Option<String>,
// }

fn main() -> anyhow::Result<()> {
    // let opt = Opt::from_args();

    match launch(runner::run) {
        Ok(_items) => stdout("foo"),
        Err(e) => stderr(e),
    }
}
