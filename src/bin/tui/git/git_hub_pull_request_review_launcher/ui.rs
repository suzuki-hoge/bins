use std::fs::File;

use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};

use bins::libs::app::multi_fix_app::MultiFixApp;
use bins::libs::ui::paragraph::create_guide;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, List, ListItem, Paragraph};
use tui::Frame;

use crate::pull_request::PullRequest;
use bins::libs::ui::spans::pane1_highlight_spans;

const PROMPT: &str = "> ";

pub fn get_height(frame: &Frame<CrosstermBackend<File>>) -> u16 {
    mk_layout(frame)[2].height - 2 // top border + bottom border
}

pub fn draw(
    frame: &mut Frame<CrosstermBackend<File>>,
    current_tab: usize,
    app: &MultiFixApp<PullRequest>,
    guide: Vec<anyhow::Result<String, String>>,
) {
    // layout

    let layout = mk_layout(frame);
    frame.set_cursor(frame.size().x + (PROMPT.len() + app.filter_input_app.cursor) as u16, frame.size().y);

    // input area

    let paragraph = Paragraph::new(format!("{}{}", PROMPT, &app.filter_input_app.input));
    frame.render_widget(paragraph, layout[0]);

    // title

    let block = Block::default().title("  Filter  ").borders(Borders::ALL).border_type(BorderType::Rounded);
    let title = create_title(current_tab).block(block);
    frame.render_widget(title, layout[1]);

    // items

    let items: Vec<ListItem> = app
        .scrolling_select_app
        .get_matched_items_in_page()
        .iter()
        .map(|&(item_number, item)| {
            pane1_highlight_spans(
                item.clone(),
                app.scrolling_select_app.is_active_item_number(item_number),
                layout[2].width,
            )
        })
        .map(ListItem::new)
        .collect();
    let block = Block::default().title("  Pull Requests  ").borders(Borders::ALL).border_type(BorderType::Rounded);
    let list = List::new(items).block(block);
    frame.render_widget(list, layout[2]);

    // guide

    let guide_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(layout[3]);
    frame.render_widget(create_guide(guide), guide_area[1]);
}

fn mk_layout(frame: &Frame<CrosstermBackend<File>>) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Length(3), Constraint::Min(1), Constraint::Length(1)])
        .split(frame.size())
}

fn create_title(current_tab: usize) -> Paragraph<'static> {
    let color = |n| if n == current_tab { Color::Black } else { Color::Rgb(190, 190, 190) };
    let span1 = Span::styled("  All", Style::default().fg(color(1)).add_modifier(Modifier::BOLD));
    let span2 = Span::styled("Own", Style::default().fg(color(2)).add_modifier(Modifier::BOLD));
    let span3 = Span::styled("Not-Reviewed", Style::default().fg(color(3)).add_modifier(Modifier::BOLD));
    let span4 = Span::styled("Reviewed  ", Style::default().fg(color(4)).add_modifier(Modifier::BOLD));
    let gap = Span::raw("  |  ");
    Paragraph::new(Spans::from(vec![span1, gap.clone(), span2, gap.clone(), span3, gap.clone(), span4]))
}
