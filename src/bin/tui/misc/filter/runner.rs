extern crate bins;

use crossterm::event::read;
use std::fs::File;
use std::io;
use termion::event::Key;
use termion::get_tty;
use termion::input::TermRead;

use bins::libs::app::multi_fix_app::MultiFixApp;
use bins::libs::key::input_filter_dispatcher::Mode::Insert;
use bins::libs::key::input_filter_dispatcher::{
    edit, exit, horizontal_move, mode_change, vertical_move, EXIT_KEYS, HORIZONTAL_MOVE_KEYS, MODE_CHANGE_KEYS,
    VERTICAL_MOVE_KEYS,
};
use bins::libs::matcher::string_matcher::Mode;
use bins::libs::util::tmp_log::tmp_log;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use ui::draw;

use crate::ui;
use crate::ui::get_height;

pub fn run(terminal: &mut Terminal<CrosstermBackend<File>>, items: Vec<String>) -> io::Result<Vec<String>> {
    let height = get_height(&terminal.get_frame());
    let mut app = MultiFixApp::init(items, height, Mode::ORIGIN);

    terminal.draw(|frame| draw(frame, &mut app))?;

    let mut mode = Insert;

    tmp_log(read()?);
    for key in get_tty()?.keys() {
        match key.unwrap() {
            key if HORIZONTAL_MOVE_KEYS.contains(&key) => horizontal_move(&mut app, key),
            key if VERTICAL_MOVE_KEYS.contains(&key) => vertical_move(&mut app, key),
            key if MODE_CHANGE_KEYS.contains(&key) => mode = mode_change(&mut app, key),
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

            key if mode == Insert => edit(&mut app, key),

            _ => {}
        }

        terminal.draw(|frame| draw(frame, &mut app))?;
    }

    Ok(vec![])
}
