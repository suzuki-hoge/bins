use itertools::Itertools;
use std::fs::File;

use tui::backend::CrosstermBackend;
use tui::layout::Rect;

use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, Paragraph};
use tui::Frame;

use crate::fuzzy::core::tab::{Tab, TabNames};

use crate::fuzzy::state::tab_state::TabState;

pub fn render_tabs(frame: &mut Frame<CrosstermBackend<File>>, rect: Rect, tab_names: &TabNames, state: &TabState) {
    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);

    let paragraph = Paragraph::new(Spans::from(
        tab_names.names.iter().enumerate().map(|(i, name)| create_span(i, name, &state.tab)).collect_vec(),
    ))
    .block(block);

    frame.render_widget(paragraph, rect);
}

fn create_span(i: usize, name: &str, tab: &Tab) -> Span<'static> {
    if i == tab.current {
        Span::styled(name.to_owned(), Style::default().fg(Color::Black).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(name.to_owned(), Style::default().fg(Color::Rgb(190, 190, 190)).add_modifier(Modifier::BOLD))
    }
}
