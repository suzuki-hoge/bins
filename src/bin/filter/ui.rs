use std::io::Stdout;

use itertools::Itertools;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{List, ListItem, Paragraph};
use tui::Frame;

use bins::apps::matched_string::MatchedString;

use crate::app::App;

const PROMPT: &str = "> ";

pub fn draw(frame: &mut Frame<CrosstermBackend<Stdout>>, app: &mut App) {
    // layout

    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(1), Constraint::Min(1)])
        .split(frame.size());

    frame.set_cursor(
        frame.size().x + (PROMPT.len() + app.input.cursor) as u16,
        frame.size().y,
    );

    // input area

    let paragraph = Paragraph::new(format!("{}{}", PROMPT, &app.input.input));
    frame.render_widget(paragraph, layout[0]);

    // lines area

    let items: Vec<ListItem> = if app.input.input.is_empty() {
        // empty input
        app.lines
            .iter()
            .enumerate()
            .map(|(i, line)| {
                ListItem::new(Spans::from(Span::styled(
                    line,
                    Style::default().bg(if app.cursor == i {
                        Color::Cyan
                    } else {
                        Color::White
                    }),
                )))
            })
            .collect()
    } else {
        // find matched
        app.lines
            .iter()
            .flat_map(|line| MatchedString::matched_only(&app.input.input, line))
            .enumerate()
            .map(|(i, ms)| {
                let content: Vec<Spans> = vec![Spans::from(
                    ms.chars
                        .into_iter()
                        .map(|mc| {
                            Span::styled(
                                mc.value,
                                Style::default()
                                    .fg(if mc.matched { Color::Red } else { Color::Black })
                                    .bg(if app.cursor == i {
                                        Color::Cyan
                                    } else {
                                        Color::White
                                    }),
                            )
                        })
                        .collect_vec(),
                )];
                ListItem::new(content)
            })
            .collect()
    };

    let list = List::new(items);
    frame.render_widget(list, layout[1]);
}
