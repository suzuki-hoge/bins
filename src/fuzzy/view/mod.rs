use std::fs::File;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

use crate::fuzzy::core::item::Item;
use crate::fuzzy::core::tab::TabNames;
use crate::fuzzy::state::State;
use crate::fuzzy::view::list_view::render_list;
use crate::fuzzy::view::preview_view::render_preview;
use crate::fuzzy::view::prompt_view::render_prompt;
use crate::fuzzy::view::tab_view::render_tabs;

mod list_view;
mod preview_view;
mod prompt_view;
mod tab_view;

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

        render_list(frame, layout[1], &state.list_state);
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

        render_list(frame, sub_layout[0], &state.list_state);

        render_preview(frame, sub_layout[1], &state.list_state);
    }
}

pub struct TabView {
    tab_names: TabNames,
}

impl TabView {
    pub fn new(tab_names: TabNames) -> Self {
        Self { tab_names }
    }
}

impl View for TabView {
    fn render<I: Item>(&self, frame: &mut Frame<CrosstermBackend<File>>, state: &State<I>) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(3), Constraint::Min(1)])
            .split(frame.size());

        render_prompt(frame, layout[0], &state.prompt_state);

        render_tabs(frame, layout[1], &self.tab_names, &state.tab_state);

        render_list(frame, layout[2], &state.list_state);
    }
}
