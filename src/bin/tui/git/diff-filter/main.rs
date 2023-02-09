extern crate bins;

use itertools::Itertools;
use regex::Regex;
use structopt::StructOpt;

use bins::libs::io::writer::stdout;
use bins::libs::launcher::crossterm_launcher::launch;
use bins::libs::process::command::{get_command_out_lines, run_command};

mod runner;
mod ui;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "s", long = "--staged", help = "show staged diff-filter")]
    staged: bool,

    #[structopt(short = "a", long = "--all", help = "show all diff-filter")]
    all: bool,
}

fn main() -> anyhow::Result<()> {
    run_command("git config --global color.diff-filter always")?;

    let opt = Opt::from_args();

    let _ = match (opt.all, opt.staged) {
        (true, true) => stdout("git diff --staged".to_string()),
        (true, false) => stdout("git diff".to_string()),
        (false, true) => stdout(format!("git diff --staged {}", select_status_lines(true)?)),
        (false, false) => stdout(format!("git diff {}", select_status_lines(false)?)),
    };

    run_command("git config --global color.diff-filter auto")
}

fn select_status_lines(staged: bool) -> anyhow::Result<String> {
    let lines = launch(|terminal| runner::run(terminal, get_status_lines(staged)?));
    Ok(match lines {
        Ok(lines) => parse_paths(lines).join(" "),
        Err(_) => String::new(),
    })
}

fn get_status_lines(staged: bool) -> anyhow::Result<Vec<String>> {
    let lines = get_command_out_lines("git status --short")?;
    Ok(match staged {
        true => lines.into_iter().filter(|line| Regex::new(r"^[MARCD]").unwrap().is_match(line)).collect_vec(),
        false => lines.into_iter().filter(|line| Regex::new(r"^ [MARCD]").unwrap().is_match(line)).collect_vec(),
    })
}

fn parse_paths(lines: Vec<String>) -> Vec<String> {
    lines.iter().map(|line| line[3..].to_string()).collect_vec()
}
