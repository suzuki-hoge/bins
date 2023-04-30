use std::fs::File;

use itertools::Itertools;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::widgets::{List, ListItem};
use tui::Frame;

use crate::fuzzy::item::item::Item;
use crate::fuzzy::state::state::State;

pub struct View {}

impl View {
    pub fn render<I: Item>(&self, frame: &mut Frame<CrosstermBackend<File>>, state: &State<I>) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)])
            .split(frame.size());

        let lines = state
            .items
            .iter()
            .filter(|item| item.get_line().contains(&state.prompt_state.input))
            .map(|item| item.get_line())
            .map(ListItem::new)
            .collect_vec();
        frame.render_widget(List::new(lines), layout[1]);
    }
}
