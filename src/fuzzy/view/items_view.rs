use std::fs::File;

use itertools::Itertools;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::widgets::{Block, BorderType, Borders, List, ListItem};
use tui::Frame;

use crate::fuzzy::item::Item;
use crate::fuzzy::state::items_state::ItemsState;

pub fn render_items<I: Item>(frame: &mut Frame<CrosstermBackend<File>>, rect: Rect, state: &ItemsState<I>) {
    let list_items = state.get_matched_lines().into_iter().map(ListItem::new).collect_vec();

    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);

    let list = List::new(list_items).block(block);

    frame.render_widget(list, rect);
}
