use std::fs::File;

use itertools::Itertools;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::widgets::{Block, BorderType, Borders, List, ListItem};
use tui::Frame;

use crate::fuzzy::core::item::Item;

use crate::fuzzy::state::list_state::ListState;

pub fn render_list<I: Item>(frame: &mut Frame<CrosstermBackend<File>>, rect: Rect, state: &ListState<I>) {
    let list_items = state.get_matched_lines(rect.height).into_iter().map(ListItem::new).collect_vec();

    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);

    let list = List::new(list_items).block(block);

    frame.render_widget(list, rect);
}
