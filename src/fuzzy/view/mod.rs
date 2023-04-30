use std::fs::File;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

use crate::fuzzy::item::Item;
use crate::fuzzy::state::State;
use crate::fuzzy::view::items_view::render_items;
use crate::fuzzy::view::preview_view::render_preview;
use crate::fuzzy::view::prompt_view::render_prompt;

mod items_view;
mod preview_view;
mod prompt_view;

pub trait View {
    fn render<I: Item>(&self, frame: &mut Frame<CrosstermBackend<File>>, state: &State<I>);
}

pub struct SimpleView {}

impl SimpleView {
    pub fn init() -> Self {
        Self {}
    }
}

impl View for SimpleView {
    fn render<I: Item>(&self, frame: &mut Frame<CrosstermBackend<File>>, state: &State<I>) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)])
            .split(frame.size());

        render_prompt(frame, layout[0], &state.prompt_state);

        render_items(frame, layout[1], &state.items_state);
    }
}

pub struct PanesView {
    direction: Direction,
    constraint: Constraint,
}

impl PanesView {
    pub fn new(direction: Direction, constraint: Constraint) -> Self {
        Self { direction, constraint }
    }
}

impl View for PanesView {
    fn render<I: Item>(&self, frame: &mut Frame<CrosstermBackend<File>>, state: &State<I>) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Min(1)])
            .split(frame.size());

        let sub_layout = Layout::default()
            .direction(self.direction.clone())
            .constraints([self.constraint, Constraint::Min(1)])
            .split(layout[1]);

        render_prompt(frame, layout[0], &state.prompt_state);

        render_items(frame, sub_layout[0], &state.items_state);

        render_preview(frame, sub_layout[1], &state.items_state);
    }
}
