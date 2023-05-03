use crate::fuzzy::command::Command;
use crate::fuzzy::command::Command::*;
use crate::fuzzy::core::item::Item;
use crate::fuzzy::core::tab::Tab;
use crate::fuzzy::state::list_state::ListState;
use crate::fuzzy::state::prompt_state::PromptState;
use crate::fuzzy::state::tab_state::TabState;

pub mod list_state;
pub mod prompt_state;
pub mod tab_state;

#[derive(Debug)]
pub struct State<I: Item> {
    pub prompt_state: PromptState,
    pub list_state: ListState<I>,
    pub tab_state: TabState,
}

impl<I: Item> State<I> {
    pub fn new(items: Vec<I>, tab: Tab) -> Self {
        let mut slf = Self {
            prompt_state: PromptState::init(),
            list_state: ListState::new(items),
            tab_state: TabState::new(tab),
        };
        slf.list_state.rematch(&slf.prompt_state.input, &slf.tab_state.tab);
        slf
    }

    pub fn dispatch(&mut self, command: Command) -> bool {
        match command {
            InsertCommand { c } => {
                self.prompt_state.insert(c);
                self.list_state.rematch(&self.prompt_state.input, &self.tab_state.tab);
                false
            }
            RemoveCommand => {
                self.prompt_state.remove();
                self.list_state.rematch(&self.prompt_state.input, &self.tab_state.tab);
                false
            }
            CutCommand => {
                self.prompt_state.cut();
                self.list_state.rematch(&self.prompt_state.input, &self.tab_state.tab);
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
                self.tab_state.tab.next();
                false
            }
            PrevTabCommand => {
                self.tab_state.tab.prev();
                false
            }
            QuitCommand => true,

            _ => false,
        }
    }
}
