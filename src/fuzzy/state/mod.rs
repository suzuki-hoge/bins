use crate::fuzzy::command::Command;
use crate::fuzzy::command::Command::*;
use crate::fuzzy::core::guide::Guide;
use crate::fuzzy::core::item::Item;
use crate::fuzzy::core::tab::{Tab, TabNames};
use crate::fuzzy::state::guide_state::GuideState;
use crate::fuzzy::state::list_state::ListState;
use crate::fuzzy::state::prompt_state::PromptState;
use crate::fuzzy::state::tab_state::TabState;
use std::borrow::Borrow;

pub mod guide_state;
pub mod list_state;
pub mod prompt_state;
pub mod tab_state;

#[derive(Debug)]
pub struct State<I: Item> {
    pub prompt_state: PromptState,
    pub list_state: ListState<I>,
    pub tab_state: Option<TabState>,
    pub guide_state: Option<GuideState>,
}

impl<I: Item> State<I> {
    pub fn new(items: Vec<I>) -> Self {
        Self {
            prompt_state: PromptState::init(),
            list_state: ListState::new(items),
            tab_state: None,
            guide_state: None,
        }
    }

    pub fn tab(mut self, tab_names: &TabNames) -> Self {
        self.tab_state = Some(TabState::new(Tab::new(tab_names)));
        self
    }

    pub fn guide(mut self, guide: Guide) -> Self {
        self.guide_state = Some(GuideState::new(guide.labels));
        self
    }

    pub fn rematch(&mut self) {
        if let Some(tab_state) = self.tab_state.borrow() {
            self.list_state.rematch(&self.prompt_state.input, Some(&tab_state.tab));
        } else {
            self.list_state.rematch(&self.prompt_state.input, None);
        }
    }

    pub fn get_result(self) -> (Vec<I>, Vec<char>) {
        let items = self.list_state.get_selected_items();
        let chars = self.guide_state.as_ref().map(|state| state.get_active_chars()).unwrap_or(vec![]);
        (items, chars)
    }

    pub fn dispatch(&mut self, command: Command) -> bool {
        match command {
            InsertCommand { c } => {
                self.prompt_state.insert(c);
                self.rematch();
            }
            RemoveCommand => {
                self.prompt_state.remove();
                self.rematch();
            }
            CutCommand => {
                self.prompt_state.cut();
                self.rematch();
            }

            RightMoveCommand => self.prompt_state.right(),
            LeftMoveCommand => self.prompt_state.left(),

            TopMoveCommand => self.prompt_state.top(),
            EndMoveCommand => self.prompt_state.end(),

            UpMoveCommand => self.list_state.up(),
            DownMoveCommand => self.list_state.down(),

            SelectCommand => self.list_state.select(),

            NextTabCommand => {
                if let Some(tab_state) = self.tab_state.as_mut() {
                    tab_state.tab.next();
                    self.rematch();
                }
            }
            PrevTabCommand => {
                if let Some(tab_state) = self.tab_state.as_mut() {
                    tab_state.tab.prev();
                    self.rematch();
                }
            }

            GuideCommand { c } => {
                if let Some(guide_state) = self.guide_state.as_mut() {
                    if let Some(item) = self.list_state.get_active_item() {
                        guide_state.toggle(item, c);
                    }
                }
            }

            FixCommand => self.list_state.select(),

            QuitCommand => {}

            _ => {}
        };

        matches!(command, FixCommand | QuitCommand)
    }
}
