use std::fs::File;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};

use bins::libs::app::multi_fix_app::MultiFixApp;
use tui::widgets::{List, ListItem, Paragraph};
use tui::Frame;

use bins::libs::ui::spans::checked_string_spans;

const PROMPT: &str = "> ";

pub fn get_height(frame: &Frame<CrosstermBackend<File>>) -> u16 {
    mk_layout(frame)[1].height
}

pub fn draw(frame: &mut Frame<CrosstermBackend<File>>, app: &mut MultiFixApp<String>) {
    // layout

    let layout = mk_layout(frame);
    frame.set_cursor(frame.size().x + (PROMPT.len() + app.input_app.cursor) as u16, frame.size().y);

    // input area

    let paragraph = Paragraph::new(format!("{}{}", PROMPT, &app.input_app.input));
    frame.render_widget(paragraph, layout[0]);

    // lines area

    let items: Vec<ListItem> = app
        .scrolling_select_app
        .get_matched_items_in_page()
        .iter()
        .map(|&(item_number, item)| {
            ListItem::new(checked_string_spans(
                item.clone(),
                app.scrolling_select_app.is_active_item_number(item_number),
                layout[1].width,
            ))
        })
        .collect();
    let list = List::new(items);
    frame.render_widget(list, layout[1]);
}

fn mk_layout(frame: &Frame<CrosstermBackend<File>>) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(frame.size())
}
