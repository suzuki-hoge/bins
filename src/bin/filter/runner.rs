extern crate bins;

use std::io;
use std::io::Stdout;

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app;
use crate::ui;

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<Vec<String>> {
    let mut app = app::App::init(
        vec![
            "youtube",
            "github",
            "twitter",
            "facebook",
            "instagram",
            "slack",
            "chatwork",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
    );

    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        if let Event::Key(e) = event::read()? {
            match e.modifiers {
                KeyModifiers::CONTROL => {
                    match e.code {
                        // horizontal moving
                        KeyCode::Char('b') => app.input.left(),
                        KeyCode::Char('f') => app.input.right(),
                        KeyCode::Char('a') => app.input.top(),
                        KeyCode::Char('e') => app.input.end(),

                        // vertical moving
                        KeyCode::Char('n') => app.down(),
                        KeyCode::Char('p') => app.up(),

                        // editing
                        KeyCode::Char('k') => app.input.cut(),
                        KeyCode::Char('c') => return Ok(vec!["".to_string()]),

                        // action
                        KeyCode::Char(' ') => return Ok(app.finish()),
                        _ => {}
                    }
                }
                _ => {
                    match e.code {
                        // horizontal moving
                        KeyCode::Left => app.input.left(),
                        KeyCode::Right => app.input.right(),

                        // vertical moving
                        KeyCode::Down => app.down(),
                        KeyCode::Up => app.up(),

                        // action
                        KeyCode::Char(' ') => app.fix(),
                        KeyCode::Enter => {
                            app.fix();
                            return Ok(app.finish());
                        }

                        // editing
                        KeyCode::Char(c) => {
                            app.input.insert(c);
                            app.refresh();
                        }
                        KeyCode::Backspace => {
                            app.input.remove();
                            app.refresh();
                        }

                        _ => {}
                    }
                }
            }
        }
    }
}
