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

    pub fn dispatch(&mut self, command: Command) -> bool {
        match command {
            InsertCommand { c } => {
                self.prompt_state.insert(c);
                self.rematch();
                false
            }
            RemoveCommand => {
                self.prompt_state.remove();
                self.rematch();
                false
            }
            CutCommand => {
                self.prompt_state.cut();
                self.rematch();
                false
            }
            RightMoveCommand => {
                self.prompt_state.right();
                false
            }
            LeftMoveCommand => {
                self.prompt_state.left();
                false
            }
            TopMoveCommand => {
                self.prompt_state.top();
                false
            }
            EndMoveCommand => {
                self.prompt_state.end();
                false
            }
            UpMoveCommand => {
                self.list_state.up();
                false
            }
            DownMoveCommand => {
                self.list_state.down();
                false
            }
            NextTabCommand => {
                if let Some(tab_state) = self.tab_state.as_mut() {
                    tab_state.tab.next();
                }
                false
            }
            PrevTabCommand => {
                if let Some(tab_state) = self.tab_state.as_mut() {
                    tab_state.tab.prev();
                }
                false
            }
            GuideCommand { c } => {
                if let Some(guide_state) = self.guide_state.as_mut() {
                    guide_state.toggle(self.list_state.get_active_item(), c);
                }
                false
            }
            QuitCommand => true,

            _ => false,
        }
    }
}
