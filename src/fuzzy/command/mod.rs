use termion::event::Key;

use crate::fuzzy::command::Command::{
    Cut, Fix, Ignore, Insert, MoveDown, MoveEnd, MoveRight, MoveTop, MoveUp, NextTab, PrevTab, Quit, Remove, Select,
    SwitchGuide,
};
use crate::fuzzy::command::CommandType::{
    GuideSwitch, HorizontalMove, ListFilter, MultiSelect, PreviewFilter, TabSwitch, VerticalMove,
};

#[derive(Eq, PartialEq)]
pub enum CommandType {
    ListFilter,
    PreviewFilter,
    HorizontalMove,
    VerticalMove,
    MultiSelect,
    TabSwitch,
    GuideSwitch,
}

#[derive(Debug)]
pub enum Command {
    Insert { c: char, preview: bool },
    Remove { preview: bool },
    Cut { preview: bool },

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
            Key::Char(c) if types.contains(&ListFilter) && !c.is_ascii_uppercase() && c != '\t' && c != '\n' => {
                Insert { c, preview: false }
            }
            Key::Backspace if types.contains(&ListFilter) => Remove { preview: false },
            Key::Ctrl('k') if types.contains(&ListFilter) => Cut { preview: false },

            Key::Char(c) if types.contains(&PreviewFilter) && !c.is_ascii_uppercase() && c != '\t' && c != '\n' => {
                Insert { c, preview: true }
            }
            Key::Backspace if types.contains(&PreviewFilter) => Remove { preview: true },
            Key::Ctrl('k') if types.contains(&PreviewFilter) => Cut { preview: true },

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
