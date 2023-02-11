extern crate bins;

use std::fs::File;

use chrono::Utc;
use termion::event::Key;
use termion::get_tty;
use termion::input::TermRead;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use ui::draw;

use crate::day::{get_weeks, next_page, prev_page};
use crate::ui;

pub fn run(terminal: &mut Terminal<CrosstermBackend<File>>) -> anyhow::Result<()> {
    let today = Utc::now();
    let mut request = Utc::now();

    terminal.draw(|frame| draw(frame, request, get_weeks(today, request)))?;

    for key in get_tty()?.keys() {
        match key.unwrap() {
            Key::Char('k') => request = prev_page(request),
            Key::Char('j') => request = next_page(request),
            Key::Ctrl('c') => return Ok(()),
            _ => {}
        }

        terminal.draw(|frame| draw(frame, request, get_weeks(today, request)))?;
    }

    unreachable!();
}
