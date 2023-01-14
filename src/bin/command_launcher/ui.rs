use itertools::Itertools;
use std::fs::File;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};

use tui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};
use tui::Frame;

use crate::ParsedContent;
use bins::libs::app::multi_fix_app::MultiFixApp;
use bins::libs::ui::spans::{checked_string_origin_spans, checked_string_preview_spans};

const PROMPT: &str = "> ";

pub fn get_height(frame: &Frame<CrosstermBackend<File>>) -> u16 {
    mk_layout(frame)[1].height - 2 // top border + bottom border
}

pub fn draw(frame: &mut Frame<CrosstermBackend<File>>, app: &mut MultiFixApp<ParsedContent>) {
    // layout

    let layout = mk_layout(frame);
    frame.set_cursor(frame.size().x + (PROMPT.len() + app.input_app.cursor) as u16, frame.size().y);

    let sub_layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(layout[1]);

    // input area

    let paragraph = Paragraph::new(format!("{}{}", PROMPT, &app.input_app.input));
    frame.render_widget(paragraph, layout[0]);

    // lines area

    let items: Vec<ListItem> = app
        .scrolling_select_app
        .get_matched_items_in_page()
        .iter()
        .map(|&(item_number, item)| {
            checked_string_origin_spans(
                item.clone(),
                app.scrolling_select_app.is_active_item_number(item_number),
                sub_layout[0].width,
            )
        })
        .map(ListItem::new)
        .collect();
    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);
    let list = List::new(items).block(block);
    frame.render_widget(list, sub_layout[0]);

    // preview area

    let items: Vec<ListItem> = app
        .scrolling_select_app
        .get_active_item()
        .map(|item| checked_string_preview_spans(item).into_iter().map(ListItem::new).collect_vec())
        .unwrap_or_default();
    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);
    let list = List::new(items).block(block);
    frame.render_widget(list, sub_layout[1]);
}

fn mk_layout(frame: &Frame<CrosstermBackend<File>>) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(frame.size())
}
