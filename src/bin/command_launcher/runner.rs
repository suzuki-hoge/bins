extern crate bins;

use std::fs::File;
use std::io;

use tui::backend::CrosstermBackend;
use tui::Terminal;

use bins::libs::app::multi_fix_app::MultiFixApp;
use bins::libs::key::input_filter_dispatcher::dispatch;

use crate::command::parsed_command::ParsedContent;
use crate::ui::{draw, get_height};

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<File>>,
    items: Vec<ParsedContent>,
) -> io::Result<Vec<ParsedContent>> {
    let height = get_height(&terminal.get_frame());
    let mut app = MultiFixApp::init(items, height);

    terminal.draw(|frame| draw(frame, &mut app))?;

    dispatch(terminal, Box::new(draw), &mut app)
}
