use crate::libs::matcher::string_matcher::CheckedString;
use itertools::Itertools;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};

pub fn checked_string_spans(cs: CheckedString, is_active: bool, max_width: u16) -> Vec<Spans<'static>> {
    let spans = cs
        .get_string_parts(max_width as usize)
        .into_iter()
        .map(|(s, highlight)| Span::styled(s, style(highlight, is_active)))
        .collect_vec();

    vec![Spans::from(spans)]
}

fn style(is_highlight: bool, is_active: bool) -> Style {
    Style::default().fg(fg(is_highlight)).bg(bg(is_active))
}

fn fg(is_highlight: bool) -> Color {
    if is_highlight {
        Color::Red
    } else {
        Color::Black
    }
}

fn bg(is_active: bool) -> Color {
    if is_active {
        Color::Cyan
    } else {
        Color::White
    }
}
