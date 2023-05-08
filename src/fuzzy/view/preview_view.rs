use std::fs::File;

use itertools::Itertools;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, List, ListItem};
use tui::Frame;

use crate::fuzzy::core::item::Item;
use crate::fuzzy::core::style::CustomPreviewStyle;
use crate::fuzzy::state::list_state::ListState;

pub fn render_preview<I: Item, S: CustomPreviewStyle>(
    frame: &mut Frame<CrosstermBackend<File>>,
    rect: Rect,
    state: &ListState<I>,
    custom_preview_style: &S,
) {
    let list_items = if custom_preview_style.is_default() {
        state
            .get_matched_preview_parts()
            .into_iter()
            .map(|parts| {
                ListItem::new(Spans::from(
                    parts.into_iter().map(|(s, is_matched)| Span::styled(s, get_style(is_matched))).collect_vec(),
                ))
            })
            .collect_vec()
    } else {
        state.get_simple_preview().into_iter().map(|s| custom_preview_style.to_list_item(s)).collect_vec()
    };

    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);

    let list = List::new(list_items).block(block);

    frame.render_widget(list, rect);
}

fn get_style(is_matched: bool) -> Style {
    match is_matched {
        true => Style::default().fg(Color::Red).bg(Color::White).add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        false => Style::default().fg(Color::Black).bg(Color::White),
    }
}
