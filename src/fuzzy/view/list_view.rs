use std::fs::File;

use itertools::Itertools;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, List, ListItem};
use tui::Frame;
use LineStatus::Active;

use crate::fuzzy::core::item::Item;
use crate::fuzzy::state::list_state::LineStatus::{ActiveSelected, Normal, Selected};
use crate::fuzzy::state::list_state::{LineStatus, ListState};

pub fn render_list<I: Item>(frame: &mut Frame<CrosstermBackend<File>>, rect: Rect, state: &ListState<I>) {
    let list_items = state
        .get_matched_line_parts(rect.height - 2) // top border & bottom border
        .into_iter()
        .map(|(line_status, mut parts)| {
            parts.push((" ".repeat(200), false));
            (line_status, parts)
        })
        .map(|(line_status, parts)| {
            ListItem::new(Spans::from(
                parts
                    .into_iter()
                    .map(|(s, is_matched)| Span::styled(s, get_style(&line_status, is_matched)))
                    .collect_vec(),
            ))
        })
        .collect_vec();

    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);

    let list = List::new(list_items).block(block);

    frame.render_widget(list, rect);
}

fn get_style(line_status: &LineStatus, is_matched: bool) -> Style {
    match (line_status, is_matched) {
        (Active, true) => {
            Style::default().fg(Color::Red).bg(Color::Cyan).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        }
        (Normal, true) => {
            Style::default().fg(Color::Red).bg(Color::White).add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
        }
        (Active, false) => Style::default().fg(Color::Black).bg(Color::Cyan),
        (Normal, false) => Style::default().fg(Color::Black).bg(Color::White),
        (Selected, _) => Style::default().fg(Color::Rgb(190, 190, 190)).bg(Color::White),
        (ActiveSelected, _) => Style::default().fg(Color::Rgb(190, 190, 190)).bg(Color::Cyan),
    }
}
