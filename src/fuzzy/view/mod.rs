use std::fs::File;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::Frame;

use crate::fuzzy::core::item::Item;
use crate::fuzzy::core::style::CustomPreviewStyle;
use crate::fuzzy::core::tab::TabNames;
use crate::fuzzy::state::State;
use crate::fuzzy::view::guide_view::render_guide;
use crate::fuzzy::view::list_view::render_list;
use crate::fuzzy::view::preview_view::render_preview;
use crate::fuzzy::view::prompt_view::render_prompt;
use crate::fuzzy::view::tab_view::render_tabs;

mod guide_view;
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

pub struct PanesView<S: CustomPreviewStyle> {
    sub_direction: Direction,
    sub_constraint: Constraint,
    custom_preview_style: S,
}

impl<S: CustomPreviewStyle> PanesView<S> {
    pub fn new(sub_direction: Direction, sub_constraint: Constraint, custom_preview_style: S) -> Self {
        Self { sub_direction, sub_constraint, custom_preview_style }
    }
}

impl<S: CustomPreviewStyle> View for PanesView<S> {
    fn render<I: Item>(&self, frame: &mut Frame<CrosstermBackend<File>>, state: &State<I>) {
        let constraints = if state.guide_state.is_none() {
            vec![Constraint::Length(1), Constraint::Min(1)]
        } else {
            vec![Constraint::Length(1), Constraint::Min(1), Constraint::Length(1)]
        };

        let layout = Layout::default().direction(Direction::Vertical).constraints(constraints).split(frame.size());

        let sub_layout = Layout::default()
            .direction(self.sub_direction.clone())
            .constraints([self.sub_constraint, Constraint::Min(1)])
            .split(layout[1]);

        render_prompt(frame, layout[0], &state.prompt_state);

        render_list(frame, sub_layout[0], &state.list_state);

        render_preview(frame, sub_layout[1], &state.list_state, &self.custom_preview_style);

        if state.guide_state.is_some() {
            let guide_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Length(1), Constraint::Min(1)])
                .split(layout[2]);
            render_guide(frame, guide_area[1], state.guide_state.as_ref().unwrap());
        }
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
        let constraints = if state.guide_state.is_none() {
            vec![Constraint::Length(1), Constraint::Length(3), Constraint::Min(1)]
        } else {
            vec![Constraint::Length(1), Constraint::Length(3), Constraint::Min(1), Constraint::Length(1)]
        };

        let layout = Layout::default().direction(Direction::Vertical).constraints(constraints).split(frame.size());

        render_prompt(frame, layout[0], &state.prompt_state);

        render_tabs(frame, layout[1], &self.tab_names, state.tab_state.as_ref().unwrap());

        render_list(frame, layout[2], &state.list_state);

        if state.guide_state.is_some() {
            render_guide(frame, layout[3], state.guide_state.as_ref().unwrap());
        }
    }
}
