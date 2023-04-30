use termion::event::Key;

use crate::fuzzy::command::command::Command::{IgnoreCommand, InputCommand, QuitCommand, RightMoveCommand};
use crate::fuzzy::command::command::CommandType::{HorizontalMove, Input};

#[derive(Eq, PartialEq)]
pub enum CommandType {
    Input,
    HorizontalMove,
}

pub enum Command {
    InputCommand { c: char },
    RightMoveCommand,
    QuitCommand,
    IgnoreCommand,
}

impl Command {
    pub fn create(key: Key, types: &[CommandType]) -> Self {
        match key {
            Key::Char(c) if types.contains(&Input) && c.is_ascii_lowercase() => InputCommand { c },
            Key::Left if types.contains(&HorizontalMove) => RightMoveCommand,
            Key::Ctrl('c') => QuitCommand,
            _ => IgnoreCommand,
        }
    }
}
