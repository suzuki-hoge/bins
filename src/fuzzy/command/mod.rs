use termion::event::Key;

use crate::fuzzy::command::Command::{
    Cut, Fix, Ignore, Insert, MoveDown, MoveEnd, MoveRight, MoveTop, MoveUp, NextTab, PrevTab, Quit, Remove, Select,
    SwitchGuide,
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
    Insert { c: char },
    Remove,
    Cut,

    MoveLeft,
    MoveRight,
    MoveTop,
    MoveEnd,

    MoveUp,
    MoveDown,

    Select,

    NextTab,
    PrevTab,

    SwitchGuide { c: char },

    Fix,

    Quit,

    Ignore,
}

impl Command {
    pub fn create(key: Key, types: &[CommandType]) -> Self {
        match key {
            Key::Char(c) if types.contains(&Input) && !c.is_ascii_uppercase() && c != '\t' && c != '\n' => Insert { c },
            Key::Backspace if types.contains(&Input) => Remove,
            Key::Ctrl('k') if types.contains(&Input) => Cut,

            Key::Left if types.contains(&HorizontalMove) => MoveRight,
            Key::Right if types.contains(&HorizontalMove) => MoveRight,
            Key::Ctrl('a') if types.contains(&HorizontalMove) => MoveTop,
            Key::Ctrl('e') if types.contains(&HorizontalMove) => MoveEnd,

            Key::Up if types.contains(&VerticalMove) => MoveUp,
            Key::Down if types.contains(&VerticalMove) => MoveDown,
            Key::Ctrl('p') if types.contains(&VerticalMove) => MoveUp,
            Key::Ctrl('n') if types.contains(&VerticalMove) => MoveDown,

            Key::Char('\t') if types.contains(&MultiSelect) => Select,

            Key::Char('\t') if types.contains(&TabSwitch) => NextTab,
            Key::BackTab if types.contains(&TabSwitch) => PrevTab,

            Key::BackTab if types.contains(&TabSwitch) => PrevTab,

            Key::Char(c) if types.contains(&GuideSwitch) && c.is_ascii_uppercase() => SwitchGuide { c },

            Key::Char('\n') => Fix,

            Key::Ctrl('c') => Quit,

            _ => Ignore,
        }
    }
}
