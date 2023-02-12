use std::fs::File;

use bins::libs::app::multi_fix_app::CursorMode::{Edit, Filter};
use itertools::Itertools;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::text::Spans;
use tui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};
use tui::Frame;

use bins::libs::app::multi_fix_app::MultiFixApp;
use bins::libs::ui::spans::{pane1_highlight_spans, pane2_highlight_spans};

use crate::project_config::ProjectItem;

const PROMPT: &str = "> ";

pub fn get_height(frame: &Frame<CrosstermBackend<File>>) -> u16 {
    mk_layout(frame)[1].height - 2 // top border + bottom border
}

pub fn draw(
    frame: &mut Frame<CrosstermBackend<File>>,
    app: &mut MultiFixApp<ProjectItem>,
    guide: &str,
    message: Result<&str, &str>,
) {
    // layout

    let layout = mk_layout(frame);

    let box_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(layout[1]);

    if app.is_filter_mode() {
        frame.set_cursor(frame.size().x + (PROMPT.len() + app.filter_input_app.cursor) as u16, frame.size().y)
    } else {
        let lines = app.edit_input_app.get();
        let x = lines.last().map(|s| s.len()).unwrap_or(0) as u16;
        let y = lines.len() as u16;
        frame.set_cursor(box_area[1].x + 1 + x, box_area[1].y + y);
    }

    // input area

    let paragraph = Paragraph::new(format!("{}{}", PROMPT, &app.filter_input_app.input));
    frame.render_widget(paragraph, layout[0]);

    // lines area

    let items: Vec<ListItem> = app
        .scrolling_select_app
        .get_matched_items_in_page()
        .iter()
        .map(|&(item_number, item)| {
            pane1_highlight_spans(
                item.clone(),
                app.scrolling_select_app.is_active_item_number(item_number),
                box_area[0].width,
            )
        })
        .map(ListItem::new)
        .collect();
    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);
    let list = List::new(items).block(block);
    frame.render_widget(list, box_area[0]);

    // preview area

    let items: Vec<ListItem> = match &app.cursor_mode {
        Filter => app
            .scrolling_select_app
            .get_active_checked_string()
            .map(|cs| pane2_highlight_spans(cs).into_iter().map(ListItem::new).collect_vec())
            .unwrap_or_default(),
        Edit => app.edit_input_app.get().into_iter().map(|line| ListItem::new(Spans::from(line))).collect_vec(),
    };
    let block = Block::default().borders(Borders::ALL).border_type(BorderType::Rounded);
    let list = List::new(items).block(block);
    frame.render_widget(list, box_area[1]);

    // guide area

    let guide_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(70), Constraint::Percentage(30)])
        .split(layout[2]);

    let p = Paragraph::new(guide);
    frame.render_widget(p, guide_area[0]);

    let p = match message {
        Ok(s) => Paragraph::new(s).style(Style::default().fg(Color::Green)).alignment(Alignment::Right),
        Err(s) => Paragraph::new(s).style(Style::default().fg(Color::Red)).alignment(Alignment::Right),
    };
    frame.render_widget(p, guide_area[1]);
}

fn mk_layout(frame: &Frame<CrosstermBackend<File>>) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1), Constraint::Length(1)])
        .split(frame.size())
}
