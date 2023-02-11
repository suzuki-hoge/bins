use chrono::{DateTime, Datelike, Utc};
use std::fs::File;

use itertools::Itertools;
use tui::backend::CrosstermBackend;
use tui::layout::{Alignment, Rect};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, BorderType, Borders, List, ListItem};
use tui::Frame;

use crate::day::Label::{Saturday, Sunday, Weekday};
use crate::day::{Day, Week};

pub fn draw(frame: &mut Frame<CrosstermBackend<File>>, request: DateTime<Utc>, weeks: Vec<Week>) {
    // layout

    let layout = Rect::new(0, 0, 24, 12);

    let spans = weeks
        .into_iter()
        .map(|week| week.into_iter().flat_map(create_spans).collect_vec())
        .map(|spans| vec![Spans::from(spans)]);
    let spans = Itertools::intersperse(spans, row_gap());
    let mut items: Vec<ListItem> = spans.map(ListItem::new).collect_vec();
    items.insert(0, ListItem::new(row_gap()));

    let block = Block::default()
        .title(format!("  {} {:>02}  ", request.year(), request.month()))
        .title_alignment(Alignment::Center)
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded);
    let list = List::new(items).block(block);

    frame.render_widget(list, layout);
}

fn create_spans(day: Day) -> Vec<Span<'static>> {
    let style = match (day.this_month, day.label) {
        (true, Sunday) => Style::default().fg(Color::Red),
        (true, Weekday) => Style::default().fg(Color::Black),
        (true, Saturday) => Style::default().fg(Color::Blue),
        (false, Sunday) => Style::default().fg(Color::Rgb(255, 170, 170)),
        (false, Weekday) => Style::default().fg(Color::Rgb(190, 190, 190)),
        (false, Saturday) => Style::default().fg(Color::Rgb(160, 160, 255)),
    };
    let span = match day.today {
        true => Span::styled(day.show(), style.add_modifier(Modifier::BOLD | Modifier::UNDERLINED)),
        false => Span::styled(day.show(), style.add_modifier(Modifier::BOLD)),
    };
    vec![col_gap(), span]
}

fn col_gap() -> Span<'static> {
    Span::from(" ")
}

fn row_gap() -> Vec<Spans<'static>> {
    vec![Spans::from(Span::raw(""))]
}
