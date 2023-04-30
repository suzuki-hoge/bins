use crate::fuzzy::command::command::Command;
use crate::fuzzy::command::command::Command::*;
use crate::fuzzy::item::item::Item;
use crate::fuzzy::state::prompt_state::PromptState;

#[derive(Debug)]
pub struct State<I: Item> {
    pub items: Vec<I>,
    pub prompt_state: PromptState,
}

impl<I: Item> State<I> {
    pub fn new(items: Vec<I>) -> Self {
        Self { items, prompt_state: PromptState::new() }
    }

    pub fn dispatch(&mut self, command: Command) -> bool {
        match command {
            InputCommand { c } => {
                self.prompt_state.input.push(c);
                false
            }
            RightMoveCommand => {
                self.prompt_state.position += 1;
                false
            }
            QuitCommand => true,
            _ => false,
        }
    }
}
