use termion::event::Key;

use crate::libs::app::multi_fix_app::MultiFixApp;
use crate::libs::item::display_item::DisplayItem;
use crate::libs::key::dispatcher::Mode::{Insert, Normal};

pub const HORIZONTAL_MOVE_KEYS: [Key; 4] = [Key::Left, Key::Right, Key::Ctrl('a'), Key::Ctrl('e')];

pub fn horizontal_move<Item>(app: &mut MultiFixApp<Item>, key: Key)
where
    Item: DisplayItem,
{
    match key {
        Key::Left => app.input_app.left(),
        Key::Right => app.input_app.right(),
        Key::Ctrl('a') => app.input_app.top(),
        Key::Ctrl('e') => app.input_app.end(),
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
pub enum Mode {
    Normal,
    Insert,
}

pub const MODE_CHANGE_KEYS: [Key; 2] = [Key::Esc, Key::Char('i')];

pub fn mode_change<Item>(_: &mut MultiFixApp<Item>, key: Key) -> Mode
where
    Item: DisplayItem,
{
    match key {
        Key::Esc => Normal,
        Key::Char('i') => Insert,
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
        Key::Char(c) => {
            app.input_app.insert(c);
            app.refresh();
        }
        Key::Backspace => {
            app.input_app.remove();
            app.refresh();
        }
        Key::Ctrl('k') => {
            app.input_app.cut();
            app.refresh();
        }
        _ => {}
    }
}
