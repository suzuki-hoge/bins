use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use termion::get_tty;
use termion::input::TermRead;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction};
use tui::Terminal;

use crate::fuzzy::command::CommandType::{
    GuideSwitch, HorizontalMove, ListFilter, MultiSelect, PreviewFilter, TabSwitch, VerticalMove,
};
use crate::fuzzy::command::{Command, CommandType};
use crate::fuzzy::core::guide::Guide;
use crate::fuzzy::core::item::Item;
use crate::fuzzy::core::style::{CustomPreviewStyle, DefaultPreviewStyle};
use crate::fuzzy::core::tab::TabNames;
use crate::fuzzy::state::State;
use crate::fuzzy::view::{PanesView, SimpleView, TabView, View};

mod command;
pub mod core;
mod logger;
mod matcher;
mod state;
mod view;

pub struct FuzzyBuilder {}

impl FuzzyBuilder {
    pub fn simple<I: Item>(items: Vec<I>) -> Simple<I> {
        Simple { items }
    }

    pub fn pane<I: Item>(items: Vec<I>, direction: Direction, constraint: Constraint) -> Pane<I> {
        Pane { items, direction, constraint }
    }

    pub fn tab<I: Item>(items: Vec<I>, names: Vec<&'static str>) -> Tab<I> {
        Tab { items, tab_names: TabNames::new(names) }
    }
}

pub struct Simple<I: Item> {
    items: Vec<I>,
}

impl<I: Item> Simple<I> {
    pub fn build(self) -> Runner<I, SimpleView> {
        let state = State::new(self.items);
        let view = SimpleView::init();
        let command_types = vec![ListFilter, HorizontalMove, VerticalMove, MultiSelect];
        Runner { state, view, command_types }
    }
}

pub struct Pane<I: Item> {
    items: Vec<I>,
    direction: Direction,
    constraint: Constraint,
}

impl<I: Item> Pane<I> {
    pub fn default_preview(self) -> PreviewPane<I, DefaultPreviewStyle> {
        PreviewPane {
            items: self.items,
            direction: self.direction,
            constraint: self.constraint,
            custom_preview_style: DefaultPreviewStyle {},
            filter_command: PreviewFilter,
        }
    }

    pub fn custom_preview<S: CustomPreviewStyle>(self, custom_preview_style: S) -> PreviewPane<I, S> {
        PreviewPane {
            items: self.items,
            direction: self.direction,
            constraint: self.constraint,
            custom_preview_style,
            filter_command: ListFilter,
        }
    }
}

pub struct PreviewPane<I: Item, S: CustomPreviewStyle> {
    items: Vec<I>,
    direction: Direction,
    constraint: Constraint,
    custom_preview_style: S,
    filter_command: CommandType,
}

impl<I: Item, S: CustomPreviewStyle> PreviewPane<I, S> {
    pub fn guide(self, labels: Vec<&'static str>, actives: Vec<usize>) -> GuidePane<I, S> {
        GuidePane {
            items: self.items,
            direction: self.direction,
            constraint: self.constraint,
            custom_preview_style: self.custom_preview_style,
            filter_command: self.filter_command,
            guide: Guide::new(labels, actives),
        }
    }

    pub fn build(self) -> Runner<I, PanesView<S>> {
        let state = State::new(self.items);
        let view = PanesView::new(self.direction, self.constraint, self.custom_preview_style);
        let command_types = vec![self.filter_command, HorizontalMove, VerticalMove, MultiSelect];
        Runner { state, view, command_types }
    }
}

pub struct GuidePane<I: Item, S: CustomPreviewStyle> {
    items: Vec<I>,
    direction: Direction,
    constraint: Constraint,
    custom_preview_style: S,
    filter_command: CommandType,
    guide: Guide,
}

impl<I: Item, S: CustomPreviewStyle> GuidePane<I, S> {
    pub fn build(self) -> Runner<I, PanesView<S>> {
        let state = State::new(self.items).guide(self.guide);
        let view = PanesView::new(self.direction, self.constraint, self.custom_preview_style);
        let command_types = vec![self.filter_command, HorizontalMove, VerticalMove, MultiSelect, GuideSwitch];
        Runner { state, view, command_types }
    }
}

pub struct Tab<I: Item> {
    items: Vec<I>,
    tab_names: TabNames,
}

impl<I: Item> Tab<I> {
    pub fn guide(self, labels: Vec<&'static str>, actives: Vec<usize>) -> GuideTab<I> {
        GuideTab { items: self.items, tab_names: self.tab_names, guide: Guide::new(labels, actives) }
    }

    pub fn build(self) -> Runner<I, TabView> {
        let state = State::new(self.items).tab(&self.tab_names);
        let view = TabView::new(self.tab_names);
        let command_types = vec![ListFilter, HorizontalMove, VerticalMove, TabSwitch];
        Runner { state, view, command_types }
    }
}

pub struct GuideTab<I: Item> {
    items: Vec<I>,
    tab_names: TabNames,
    guide: Guide,
}

impl<I: Item> GuideTab<I> {
    pub fn build(self) -> Runner<I, TabView> {
        let state = State::new(self.items).tab(&self.tab_names).guide(self.guide);
        let view = TabView::new(self.tab_names);
        let command_types = vec![ListFilter, HorizontalMove, VerticalMove, TabSwitch, GuideSwitch];
        Runner { state, view, command_types }
    }
}

pub struct Runner<I: Item, V: View> {
    state: State<I>,
    view: V,
    command_types: Vec<CommandType>,
}

impl<I: Item, V: View> Runner<I, V> {
    pub fn run(&mut self) -> anyhow::Result<(Vec<I>, Vec<char>)> {
        enable_raw_mode()?;
        let mut tty = get_tty()?;
        execute!(tty, EnterAlternateScreen)?;
        let backend = CrosstermBackend::new(tty);
        let mut terminal = Terminal::new(backend)?;

        self.state.rematch(false);
        terminal.draw(|frame| self.view.render(frame, &self.state))?;

        for key in get_tty()?.keys() {
            let key = key.unwrap();
            let command = Command::create(key, &self.command_types);
            if self.state.dispatch(command) {
                break;
            }
            terminal.draw(|frame| self.view.render(frame, &self.state))?;
        }

        disable_raw_mode()?;
        execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
        terminal.show_cursor()?;

        Ok(self.state.get_result())
    }
}
