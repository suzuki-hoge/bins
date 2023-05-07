use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use termion::get_tty;
use termion::input::TermRead;
use tui::{backend::CrosstermBackend, Terminal};

use crate::fuzzy::command::{Command, CommandType};
use crate::fuzzy::core::item::Item;
use crate::fuzzy::state::State;
use crate::fuzzy::view::View;

pub fn process<V: View, I: Item>(
    view: V,
    mut state: State<I>,
    command_types: &[CommandType],
) -> anyhow::Result<(Vec<I>, Vec<char>)> {
    enable_raw_mode()?;
    let mut tty = get_tty()?;
    execute!(tty, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(tty);
    let mut terminal = Terminal::new(backend)?;

    state.rematch();
    terminal.draw(|frame| view.render(frame, &state))?;

    for key in get_tty()?.keys() {
        let key = key.unwrap();
        let command = Command::create(key, command_types);
        if state.dispatch(command) {
            break;
        }
        terminal.draw(|frame| view.render(frame, &state))?;
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(state.get_result())
}
