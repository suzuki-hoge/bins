use itertools::Itertools;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};

use crate::libs::item::display_item::DisplayItem;
use crate::libs::matcher::string_matcher::CheckedString;

pub fn pane1_highlight_spans<Item>(cs: CheckedString<Item>, is_active: bool, max_width: u16) -> Vec<Spans<'static>>
where
    Item: DisplayItem,
{
    let spans = cs
        .get_pane1_string_parts(max_width as usize)
        .into_iter()
        .map(|(s, highlight)| Span::styled(s, style(highlight, is_active)))
        .collect_vec();

    vec![Spans::from(spans)]
}

pub fn pane2_highlight_spans<Item>(cs: CheckedString<Item>) -> Vec<Vec<Spans<'static>>>
where
    Item: DisplayItem,
{
    cs.get_pane2_string_parts_vec()
        .into_iter()
        .map(|parts| parts.into_iter().map(|(s, highlight)| Span::styled(s, style(highlight, false))).collect_vec())
        .map(|spans| vec![Spans::from(spans)])
        .collect_vec()
}

fn style(is_highlight: bool, is_active: bool) -> Style {
    let style = Style::default().fg(fg(is_highlight)).bg(bg(is_active));
    if is_highlight {
        style.add_modifier(Modifier::BOLD | Modifier::UNDERLINED)
    } else {
        style
    }
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
