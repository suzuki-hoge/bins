use std::fs::File;

use itertools::Itertools;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, List, ListItem};
use tui::Frame;

use crate::fuzzy::core::item::Item;
use crate::fuzzy::state::list_state::ListState;

pub fn render_list<I: Item>(frame: &mut Frame<CrosstermBackend<File>>, rect: Rect, state: &ListState<I>) {
    let list_items = state
        .get_matched_line_parts(rect.height - 2) // top border & bottom border
        .into_iter()
        .map(|(is_active_line, mut parts)| {
            parts.push((" ".repeat(200), false));
            (is_active_line, parts)
        })
        .map(|(is_active_line, parts)| {
            ListItem::new(Spans::from(
                parts
                    .into_iter()
                    .map(|(s, is_matched)| Span::styled(s, get_style(is_active_line, is_matched)))
                    .collect_vec(),
            ))
        })
        .collect_vec();

    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);

    let list = List::new(list_items).block(block);

    frame.render_widget(list, rect);
}

fn get_style(is_active_line: bool, is_matched: bool) -> Style {
    match (is_active_line, is_matched) {
        (true, true) => {
            Style::default().fg(Color::Red).bg(Color::Cyan).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        }
        (false, true) => {
            Style::default().fg(Color::Red).bg(Color::White).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        }
        (true, false) => Style::default().fg(Color::Black).bg(Color::Cyan),
        (false, false) => Style::default().fg(Color::Black).bg(Color::White),
    }
}
