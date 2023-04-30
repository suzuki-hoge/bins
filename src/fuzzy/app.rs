use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use termion::get_tty;
use termion::input::TermRead;
use tui::{backend::CrosstermBackend, Terminal};


use crate::fuzzy::command::command::{Command, CommandType};
use crate::fuzzy::item::item::Item;
use crate::fuzzy::state::state::State;
use crate::fuzzy::view::view::View;

pub fn process<I: Item>(view: View, mut state: State<I>, command_types: &[CommandType]) -> anyhow::Result<()> {
    enable_raw_mode()?;
    let mut tty = get_tty()?;
    execute!(tty, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(tty);
    let mut terminal = Terminal::new(backend)?;

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

    Ok(())
}
