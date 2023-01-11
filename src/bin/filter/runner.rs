extern crate bins;

use std::fs::File;
use std::io;

use bins::libs::app::multi_fix_app::MultiFixApp;
use termion::event::Key;
use termion::get_tty;
use termion::input::TermRead;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use ui::draw;

use crate::ui;
use crate::ui::get_height;

pub fn run(terminal: &mut Terminal<CrosstermBackend<File>>, lines: Vec<String>) -> io::Result<Vec<String>> {
    let height = get_height(&terminal.get_frame());
    let mut app = MultiFixApp::init(lines, height);

    let tty = get_tty()?;

    terminal.draw(|frame| draw(frame, &mut app))?;

    for key in tty.keys() {
        match key.unwrap() {
            // horizontal move
            Key::Left => app.input_app.left(),
            Key::Right => app.input_app.right(),
            Key::Ctrl('b') => app.input_app.left(),
            Key::Ctrl('f') => app.input_app.right(),
            Key::Ctrl('a') => app.input_app.top(),
            Key::Ctrl('e') => app.input_app.end(),

            // vertical move
            Key::Down => app.scrolling_select_app.down(),
            Key::Up => app.scrolling_select_app.up(),
            Key::Ctrl('n') => app.scrolling_select_app.down(),
            Key::Ctrl('p') => app.scrolling_select_app.up(),

            // exit
            Key::Ctrl('c') => return Ok(vec![]),

            // fix one and finish
            Key::Char('\n') => {
                app.fix();
                return Ok(app.finish());
            }

            // fix one
            Key::Ctrl('u') => app.fix(),

            // finish
            Key::Ctrl('o') => return Ok(app.finish()),

            // input
            Key::Char(c) => {
                app.input_app.insert(c);
                app.refresh();
            }
            Key::Backspace => {
                app.input_app.remove();
                app.refresh();
            }
            Key::Ctrl('k') => {
                app.input_app.cut();
                app.refresh();
            }

            _ => {}
        }

        terminal.draw(|frame| draw(frame, &mut app))?;
    }

    Ok(vec![])
}
