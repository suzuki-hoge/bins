extern crate bins;

use std::io;
use std::io::Stdout;

use crossterm::event;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use tui::backend::CrosstermBackend;
use tui::Terminal;

use crate::app;
use crate::ui;

pub fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> io::Result<String> {
    let mut app = app::App::init(
        vec![
            "YouTube", "youtube", "GitHub", "github", "Twitter", "twitter",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect(),
    );

    loop {
        terminal.draw(|frame| ui::draw(frame, &mut app))?;

        if let Event::Key(e) = event::read()? {
            match e.modifiers {
                KeyModifiers::CONTROL => match e.code {
                    // horizontal moving
                    KeyCode::Char('b') => {
                        app.input.left();
                    }
                    KeyCode::Char('f') => {
                        app.input.right();
                    }
                    KeyCode::Char('a') => {
                        app.input.top();
                    }
                    KeyCode::Char('e') => {
                        app.input.end();
                    }

                    // vertical moving
                    KeyCode::Char('n') => {
                        app.down();
                    }
                    KeyCode::Char('p') => {
                        app.up();
                    }

                    // editing
                    KeyCode::Char('k') => {
                        app.input.cut();
                    }
                    KeyCode::Char('c') => return Ok("".to_string()),
                    _ => {}
                },
                _ => match e.code {
                    // horizontal moving
                    KeyCode::Left => {
                        app.input.left();
                    }
                    KeyCode::Right => {
                        app.input.right();
                    }

                    // vertical moving
                    KeyCode::Down => {
                        app.down();
                    }
                    KeyCode::Up => {
                        app.up();
                    }

                    // editing
                    KeyCode::Backspace => {
                        app.input.remove();
                        app.refresh();
                    }
                    KeyCode::Char(c) => {
                        app.input.insert(c);
                        app.refresh();
                    }
                    _ => {}
                },
            }
        }
    }
}
