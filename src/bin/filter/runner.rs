extern crate bins;

use std::io;
use std::io::Stdout;

use app::App;
use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use tui::backend::CrosstermBackend;
use tui::Terminal;
use ui::draw;

use crate::app;
use crate::ui;
use crate::ui::get_height;

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, lines: Vec<String>) -> io::Result<Vec<String>> {
    let height = get_height(&terminal.get_frame());

    let mut app = App::init(lines, height);

    loop {
        terminal.draw(|frame| draw(frame, &mut app))?;

        if let Event::Key(e) = event::read()? {
            match e.modifiers {
                KeyModifiers::CONTROL => {
                    match e.code {
                        // horizontal moving
                        KeyCode::Char('b') => app.input_app.left(),
                        KeyCode::Char('f') => app.input_app.right(),
                        KeyCode::Char('a') => app.input_app.top(),
                        KeyCode::Char('e') => app.input_app.end(),

                        // vertical moving
                        KeyCode::Char('n') => app.paged_select_app.down(),
                        KeyCode::Char('p') => app.paged_select_app.up(),

                        // editing
                        KeyCode::Char('k') => app.input_app.cut(),
                        KeyCode::Char('c') => return Ok(vec!["".to_string()]),

                        // action
                        KeyCode::Char(' ') => return Ok(app.finish()),
                        _ => {}
                    }
                }
                _ => {
                    match e.code {
                        // horizontal moving
                        KeyCode::Left => app.input_app.left(),
                        KeyCode::Right => app.input_app.right(),

                        // vertical moving
                        KeyCode::Down => app.paged_select_app.down(),
                        KeyCode::Up => app.paged_select_app.up(),

                        // action
                        KeyCode::Char(' ') => app.fix(),
                        KeyCode::Enter => {
                            app.fix();
                            return Ok(app.finish());
                        }

                        // editing
                        KeyCode::Char(c) => {
                            app.input_app.insert(c);
                            app.refresh();
                        }
                        KeyCode::Backspace => {
                            app.input_app.remove();
                            app.refresh();
                        }

                        _ => {}
                    }
                }
            }
        }
    }
}
