use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};

use tui::widgets::{List, ListItem, Paragraph};
use tui::Frame;

use bins::libs::common::matched_string::MatchedString;
use bins::libs::ui::matched_string_spans::matched_string_spans;
use bins::libs::util::tmp_log::tmp_log;

use crate::app::App;

const PROMPT: &str = "> ";

pub fn draw(frame: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App) {
    // layout

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(frame.size());

    tmp_log(layout);

    frame.set_cursor(frame.size().x + (PROMPT.len() + app.input.cursor) as u16, frame.size().y);

    // input area

    let paragraph = Paragraph::new(format!("{}{}", PROMPT, &app.input.input));
    frame.render_widget(paragraph, layout[0]);

    // lines area

    let items: Vec<ListItem> = app
        .origin_lines
        .iter()
        .flat_map(|line| MatchedString::matched_only(&app.input.input, line))
        .enumerate()
        .map(|(line_number, ms)| ListItem::new(matched_string_spans(ms, app.cursor == line_number)))
        .collect();
    let list = List::new(items);
    frame.render_widget(list, layout[1]);
}
