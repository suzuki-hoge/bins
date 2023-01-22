use std::fs::File;

use crossterm::{
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use termion::get_tty;
use tui::{backend::CrosstermBackend, Terminal};

pub fn launch<Runner, Items>(runner: Runner) -> anyhow::Result<Items>
where
    Runner: Fn(&mut Terminal<CrosstermBackend<File>>) -> anyhow::Result<Items>,
{
    enable_raw_mode()?;
    let mut tty = get_tty()?;
    execute!(tty, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(tty);
    let mut terminal = Terminal::new(backend)?;

    let res = runner(&mut terminal);

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    res
}
