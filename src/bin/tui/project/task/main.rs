use crate::item::makefile_parser::MakefileParser;
use crate::item::package_json_parser::PackageJsonParser;
use crate::item::parser::{CommandType, ParsedItem, Parser};
use crate::item::task_parser::{get_config_path, TaskParser};
use bins::fuzzy::FuzzyBuilder;
use bins::io::exit::{exit_err, exit_ok};
use itertools::Itertools;
use std::env::current_dir;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;
use structopt::StructOpt;
use tui::layout::Constraint::Percentage;
use tui::layout::Direction::Horizontal;

mod item;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "editing", help = "editing file path")]
    editing: String,

    #[structopt(name = "out", help = "stdout file path")]
    out: String,

    #[structopt(name = "err", help = "stderr file path")]
    err: String,

    #[structopt(short = "e", long = "--edit", help = "edit config")]
    edit: bool,

    #[structopt(name = "label", help = "run specified item instantly")]
    label: Option<String>,
}

fn main() -> anyhow::Result<ExitCode> {
    let opt = Opt::from_args();

    match (opt.edit, opt.label) {
        (true, _) => edit(&opt.out),
        (_, Some(label)) => run(&opt.editing, &opt.out, &opt.err, label),
        (_, None) => fuzzy(&opt.editing, &opt.out, &opt.err),
    }
}

fn edit<P: AsRef<Path>>(out: P) -> anyhow::Result<ExitCode> {
    exit_ok(out, format!("vi {}", get_config_path().display()))
}

fn run<P: AsRef<Path>>(editing: P, out: P, err: P, label: String) -> anyhow::Result<ExitCode> {
    let items = TaskParser.get_items(&current_dir()?);

    match items.into_iter().find(|item| item.label == format!("bb {label}")) {
        Some(item) => exec_item(editing, out, &item),
        None => exit_err(err, "no such item"),
    }
}

fn fuzzy<P: AsRef<Path>>(editing: P, out: P, err: P) -> anyhow::Result<ExitCode> {
    let current_dir = current_dir()?;
    let parsers: Vec<Box<dyn Parser>> =
        vec![Box::new(MakefileParser), Box::new(PackageJsonParser), Box::new(TaskParser)];
    let items = parsers.iter().flat_map(|parser| parser.get_items(&current_dir)).collect();

    let (items, _) = FuzzyBuilder::pane(items, Horizontal, Percentage(30)).default_preview().build().run()?;

    match items.len() {
        1 => exec_item(editing, out, items.first().unwrap()),
        _ => exit_err(err, "no item"),
    }
}

fn exec_item<P: AsRef<Path>>(editing: P, out: P, item: &ParsedItem) -> anyhow::Result<ExitCode> {
    match item.command_type {
        CommandType::Pass => exit_ok(out, &item.label),
        CommandType::Run => exit_ok(out, item.replace_args().into_iter().join("\n")),
        CommandType::Edit => {
            let lines = item.replace_args();
            let mut file = File::create(&editing)?;
            writeln!(file, "{}", lines.into_iter().join("\n"))?;
            file.flush()?;
            exit_ok(out, format!("vi {}", editing.as_ref().display()))
        }
        CommandType::Copy => {
            let line = item.lines.first().unwrap();
            exit_ok(out, format!("echo '{line}' | tr -d '\\n' | pbcopy"))
        }
    }
}
