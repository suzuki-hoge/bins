pub mod items_state;
pub mod prompt_state;

use crate::fuzzy::command::Command;
use crate::fuzzy::command::Command::*;
use crate::fuzzy::item::Item;
use crate::fuzzy::state::items_state::ItemsState;
use crate::fuzzy::state::prompt_state::PromptState;

#[derive(Debug)]
pub struct State<I: Item> {
    pub prompt_state: PromptState,
    pub items_state: ItemsState<I>,
}

impl<I: Item> State<I> {
    pub fn new(items: Vec<I>) -> Self {
        Self { prompt_state: PromptState::init(), items_state: ItemsState::new(items) }
    }

    pub fn dispatch(&mut self, command: Command) -> bool {
        match command {
            InsertCommand { c } => {
                self.prompt_state.insert(c);
                self.items_state.update_filer(&self.prompt_state.input);
                false
            }
            RemoveCommand => {
                self.prompt_state.remove();
                self.items_state.update_filer(&self.prompt_state.input);
                false
            }
            CutCommand => {
                self.prompt_state.cut();
                self.items_state.update_filer(&self.prompt_state.input);
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
                self.items_state.up();
                false
            }
            DownMoveCommand => {
                self.items_state.down();
                false
            }
            QuitCommand => true,

            _ => false,
        }
    }
}
