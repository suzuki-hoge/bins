use crate::libs::common::matched_string::{MatchedString, Part};
use itertools::Itertools;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};

pub fn matched_string_spans(ms: MatchedString, is_highlight: bool) -> Vec<Spans<'static>> {
    let spans = ms.parts.iter().map(|p| Span::styled(p.value.clone(), style(p, is_highlight))).collect_vec();
    return vec![Spans::from(spans)];
}

fn style(p: &Part, is_highlight: bool) -> Style {
    Style::default().fg(fg(p)).bg(bg(is_highlight))
}

fn fg(p: &Part) -> Color {
    if p.matched {
        Color::Red
    } else {
        Color::Black
    }
}

fn bg(is_highlight: bool) -> Color {
    if is_highlight {
        Color::Cyan
    } else {
        Color::White
    }
}
