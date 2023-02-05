extern crate bins;

use std::fs::File;

use termion::event::Key;
use termion::get_tty;
use termion::input::TermRead;

use bins::libs::app::multi_fix_app::MultiFixApp;
use bins::libs::key::dispatcher::{
    edit, exit, horizontal_move, vertical_move, EXIT_KEYS, HORIZONTAL_MOVE_KEYS, VERTICAL_MOVE_KEYS,
};
use bins::libs::matcher::string_matcher::MatchMode;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use ui::draw;

use crate::ui;
use crate::ui::get_height;

pub fn run(terminal: &mut Terminal<CrosstermBackend<File>>, items: Vec<String>) -> anyhow::Result<Vec<String>> {
    let height = get_height(&terminal.get_frame());
    let mut app = MultiFixApp::init(items, height, MatchMode::PANE1);

    terminal.draw(|frame| draw(frame, &mut app))?;

    for key in get_tty()?.keys() {
        match key.unwrap() {
            key if HORIZONTAL_MOVE_KEYS.contains(&key) => horizontal_move(&mut app, key),
            key if VERTICAL_MOVE_KEYS.contains(&key) => vertical_move(&mut app, key),
            key if EXIT_KEYS.contains(&key) => return exit(&mut app, key),

            // fix one
            Key::Null => app.fix(),

            // finish
            Key::Ctrl('f') => return Ok(app.finish()),

            // fix one and finish
            Key::Char('\n') => {
                app.fix();
                return Ok(app.finish());
            }

            key => edit(&mut app, key),
        }

        terminal.draw(|frame| draw(frame, &mut app))?;
    }

    unreachable!();
}
