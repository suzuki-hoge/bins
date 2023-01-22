use std::fs::File;
use std::io;

use termion::event::Key;
use termion::get_tty;
use termion::input::TermRead;
use tui::backend::CrosstermBackend;
use tui::{Frame, Terminal};

use crate::libs::app::multi_fix_app::MultiFixApp;
use crate::libs::item::previewable_item::PreviewableItem;
use crate::libs::key::input_filter_dispatcher::Mode::{Insert, Normal};

pub fn dispatch<Item, Drawer>(
    terminal: &mut Terminal<CrosstermBackend<File>>,
    drawer: Box<Drawer>,
    app: &mut MultiFixApp<Item>,
) -> io::Result<Vec<Item>>
where
    Item: PreviewableItem,
    Drawer: Fn(&mut Frame<CrosstermBackend<File>>, &mut MultiFixApp<Item>),
{
    let tty = get_tty()?;

    for key in tty.keys() {
        match key.unwrap() {
            // horizontal move
            Key::Left => app.input_app.left(),
            Key::Right => app.input_app.right(),
            Key::Ctrl('b') => app.input_app.left(),
            Key::Ctrl('f') => app.input_app.right(),
            Key::Ctrl('a') => app.input_app.top(),
            Key::Ctrl('e') => app.input_app.end(),

            // vertical move
            Key::Down => app.scrolling_select_app.down(),
            Key::Up => app.scrolling_select_app.up(),
            Key::Ctrl('n') => app.scrolling_select_app.down(),
            Key::Ctrl('p') => app.scrolling_select_app.up(),

            // exit
            Key::Ctrl('c') => return Ok(vec![]),

            // fix one and finish
            Key::Char('\n') => {
                app.fix();
                return Ok(app.finish());
            }

            // fix one
            Key::Null => app.fix(),

            // finish
            Key::Char('\t') => return Ok(app.finish()),

            // input
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

        terminal.draw(|frame| drawer(frame, app))?;
    }

    Ok(vec![])
}

pub const HORIZONTAL_MOVE_KEYS: [Key; 6] =
    [Key::Left, Key::Right, Key::Ctrl('b'), Key::Ctrl('f'), Key::Ctrl('a'), Key::Ctrl('e')];

pub fn horizontal_move<Item>(app: &mut MultiFixApp<Item>, key: Key)
where
    Item: PreviewableItem,
{
    match key {
        Key::Left => app.input_app.left(),
        Key::Right => app.input_app.right(),
        Key::Ctrl('b') => app.input_app.left(),
        Key::Ctrl('f') => app.input_app.right(),
        Key::Ctrl('a') => app.input_app.top(),
        Key::Ctrl('e') => app.input_app.end(),
        _ => unreachable!(),
    }
}

pub const VERTICAL_MOVE_KEYS: [Key; 4] = [Key::Down, Key::Up, Key::Ctrl('n'), Key::Ctrl('p')];

pub fn vertical_move<Item>(app: &mut MultiFixApp<Item>, key: Key)
where
    Item: PreviewableItem,
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
    Item: PreviewableItem,
{
    match key {
        Key::Esc => Normal,
        Key::Char('i') => Insert,
        _ => unreachable!(),
    }
}

pub const EXIT_KEYS: [Key; 1] = [Key::Ctrl('c')];

pub fn exit<Item>(_: &mut MultiFixApp<Item>, key: Key) -> io::Result<Vec<Item>>
where
    Item: PreviewableItem,
{
    match key {
        Key::Ctrl('c') => Ok(vec![]),
        _ => unreachable!(),
    }
}

pub fn edit<Item>(app: &mut MultiFixApp<Item>, key: Key)
where
    Item: PreviewableItem,
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
