use termion::event::Key;

use crate::libs::app::multi_fix_app::MultiFixApp;
use crate::libs::item::display_item::DisplayItem;

pub const HORIZONTAL_MOVE_KEYS: [Key; 4] = [Key::Left, Key::Right, Key::Ctrl('a'), Key::Ctrl('e')];

pub fn horizontal_move<Item>(app: &mut MultiFixApp<Item>, key: Key)
where
    Item: DisplayItem,
{
    match key {
        Key::Left => app.filter_input_app.left(),
        Key::Right => app.filter_input_app.right(),
        Key::Ctrl('a') => app.filter_input_app.top(),
        Key::Ctrl('e') => app.filter_input_app.end(),
        _ => unreachable!(),
    }
}

pub const VERTICAL_MOVE_KEYS: [Key; 4] = [Key::Down, Key::Up, Key::Ctrl('n'), Key::Ctrl('p')];

pub fn vertical_move<Item>(app: &mut MultiFixApp<Item>, key: Key)
where
    Item: DisplayItem,
{
    match key {
        Key::Down => app.scrolling_select_app.down(),
        Key::Up => app.scrolling_select_app.up(),
        Key::Ctrl('n') => app.scrolling_select_app.down(),
        Key::Ctrl('p') => app.scrolling_select_app.up(),
        _ => unreachable!(),
    }
}

#[derive(Eq, PartialEq)]
pub enum CommandMode {
    Active,
    Inactive,
}

pub const CHANGE_COMMAND_MODE_KEYS: [Key; 1] = [Key::Esc];

pub const CURSOR_MODE_CHANGE_KEYS: [Key; 1] = [Key::Char('\t')];

pub fn change_cursor_mode<Item>(app: &mut MultiFixApp<Item>, key: Key) -> bool
where
    Item: DisplayItem,
{
    match key {
        Key::Char('\t') => app.switch_cursor_mode(),
        _ => unreachable!(),
    }
}

pub const EXIT_KEYS: [Key; 1] = [Key::Ctrl('c')];

pub fn exit<Item>(_: &mut MultiFixApp<Item>, key: Key) -> anyhow::Result<Vec<Item>>
where
    Item: DisplayItem,
{
    match key {
        Key::Ctrl('c') => Ok(vec![]),
        _ => unreachable!(),
    }
}

pub fn edit<Item>(app: &mut MultiFixApp<Item>, key: Key)
where
    Item: DisplayItem,
{
    match key {
        Key::Char(c) => app.insert(c),
        Key::Backspace => app.remove(),
        Key::Ctrl('k') => app.cut(),
        _ => {}
    }
}
