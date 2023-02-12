use itertools::Itertools;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;

pub fn create_message_paragraph(message: anyhow::Result<&str, &str>) -> Paragraph<'static> {
    match message {
        Ok(s) => Paragraph::new(s.to_string()).style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
        Err(s) => Paragraph::new(s.to_string()).style(Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
    }
}

pub fn create_guide(words: Vec<Result<String, String>>) -> Paragraph<'static> {
    let spans = words
        .into_iter()
        .map(|word| match word {
            Ok(s) => Span::styled(s, Style::default().fg(Color::Black).add_modifier(Modifier::BOLD)),
            Err(s) => Span::styled(s, Style::default().fg(Color::Gray)),
        })
        .collect_vec();

    Paragraph::new(Spans::from(spans))
}
