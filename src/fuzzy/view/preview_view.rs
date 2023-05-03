use std::fs::File;

use itertools::Itertools;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;

use tui::widgets::{Block, BorderType, Borders, List, ListItem};
use tui::Frame;

use crate::fuzzy::core::item::Item;
use crate::fuzzy::state::list_state::ListState;

pub fn render_preview<I: Item>(frame: &mut Frame<CrosstermBackend<File>>, rect: Rect, state: &ListState<I>) {
    let item = state.get_active_item();

    let list_items: Vec<ListItem> = item.get_preview().into_iter().map(|s| item.custom_preview_style(s)).collect_vec();

    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);

    let list = List::new(list_items).block(block);

    frame.render_widget(list, rect);
}
