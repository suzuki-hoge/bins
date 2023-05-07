use termion::event::Key;

use crate::fuzzy::command::Command::{
    CutCommand, DownMoveCommand, EndMoveCommand, FixCommand, GuideCommand, IgnoreCommand, InsertCommand,
    NextTabCommand, PrevTabCommand, QuitCommand, RemoveCommand, RightMoveCommand, SelectCommand, TopMoveCommand,
    UpMoveCommand,
};
use crate::fuzzy::command::CommandType::{GuideSwitch, HorizontalMove, Input, MultiSelect, TabSwitch, VerticalMove};

#[derive(Eq, PartialEq)]
pub enum CommandType {
    Input,
    HorizontalMove,
    VerticalMove,
    MultiSelect,
    TabSwitch,
    GuideSwitch,
}

#[derive(Debug)]
pub enum Command {
    InsertCommand { c: char },
    RemoveCommand,
    CutCommand,

    LeftMoveCommand,
    RightMoveCommand,
    TopMoveCommand,
    EndMoveCommand,

    UpMoveCommand,
    DownMoveCommand,

    SelectCommand,

    NextTabCommand,
    PrevTabCommand,

    GuideCommand { c: char },

    FixCommand,

    QuitCommand,

    IgnoreCommand,
}

impl Command {
    pub fn create(key: Key, types: &[CommandType]) -> Self {
        match key {
            Key::Char(c) if types.contains(&Input) && !c.is_ascii_uppercase() && c != '\t' && c != '\n' => {
                InsertCommand { c }
            }
            Key::Backspace if types.contains(&Input) => RemoveCommand,
            Key::Ctrl('k') if types.contains(&Input) => CutCommand,

            Key::Left if types.contains(&HorizontalMove) => RightMoveCommand,
            Key::Right if types.contains(&HorizontalMove) => RightMoveCommand,
            Key::Ctrl('a') if types.contains(&HorizontalMove) => TopMoveCommand,
            Key::Ctrl('e') if types.contains(&HorizontalMove) => EndMoveCommand,

            Key::Up if types.contains(&VerticalMove) => UpMoveCommand,
            Key::Down if types.contains(&VerticalMove) => DownMoveCommand,
            Key::Ctrl('p') if types.contains(&VerticalMove) => UpMoveCommand,
            Key::Ctrl('n') if types.contains(&VerticalMove) => DownMoveCommand,

            Key::Char('\t') if types.contains(&MultiSelect) => SelectCommand,

            Key::Char('\t') if types.contains(&TabSwitch) => NextTabCommand,
            Key::BackTab if types.contains(&TabSwitch) => PrevTabCommand,

            Key::BackTab if types.contains(&TabSwitch) => PrevTabCommand,

            Key::Char(c) if types.contains(&GuideSwitch) && c.is_ascii_uppercase() => GuideCommand { c },

            Key::Char('\n') => FixCommand,

            Key::Ctrl('c') => QuitCommand,

            _ => IgnoreCommand,
        }
    }
}
