extern crate bins;

use crate::command::makefile::parse_makefile;
use crate::command::parsed_command::ParsedContent;
use bins::libs::io::writer::output_or_exit;
use bins::libs::item::previewable_item::PreviewableItem;
use bins::libs::launcher::crossterm_launcher::launch;
use itertools::Itertools;

mod command;
mod runner;
mod ui;

fn main() {
    let commands = vec![parse_makefile("/tmp/Makefile".to_string())];

    let contents = commands.iter().flat_map(|command| command.get_items()).collect_vec();

    match launch(|terminal| runner::run(terminal, contents.clone())) {
        Ok(items) => output_or_exit(items.iter().map(|item| item.get_origin()).join("\n")),
        Err(e) => output_or_exit(e),
    }
}
