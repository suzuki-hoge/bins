use std::io::Stdout;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};

use tui::widgets::{List, ListItem, Paragraph};
use tui::Frame;

use bins::libs::ui::matched_string_spans::matched_string_spans;

use crate::app::App;

const PROMPT: &str = "> ";

pub fn draw(frame: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App) {
    // layout

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(frame.size());
    frame.set_cursor(frame.size().x + (PROMPT.len() + app.input_app.cursor) as u16, frame.size().y);
    app.paged_select_app.set_per_page(layout[1].height);

    // input area

    let paragraph = Paragraph::new(format!("{}{}", PROMPT, &app.input_app.input));
    frame.render_widget(paragraph, layout[0]);

    // lines area

    let items: Vec<ListItem> = app
        .paged_select_app
        .get_matched_items_in_page()
        .iter()
        .map(|(item_number, item)| {
            ListItem::new(matched_string_spans(item.clone(), app.paged_select_app.is_active_item_number(*item_number)))
        })
        .collect();
    let list = List::new(items);
    frame.render_widget(list, layout[1]);
}
