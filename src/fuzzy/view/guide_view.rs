use itertools::Itertools;
use std::fs::File;

use crate::fuzzy::core::guide::Label;
use crate::fuzzy::state::guide_state::GuideState;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use tui::widgets::Paragraph;
use tui::Frame;

pub fn render_guide(frame: &mut Frame<CrosstermBackend<File>>, rect: Rect, state: &GuideState) {
    let paragraph = Paragraph::new(Spans::from(
        Itertools::intersperse(
            state.labels.iter().map(|(label, active)| create_span(label, active)),
            create_gap_span(),
        )
        .collect_vec(),
    ));

    frame.render_widget(paragraph, rect);
}

fn create_span(label: &Label, active: &bool) -> Span<'static> {
    let s = format!("{}: {}", label.c, label.value);
    if *active {
        Span::styled(s, Style::default().fg(Color::Black).add_modifier(Modifier::BOLD))
    } else {
        Span::styled(s, Style::default().fg(Color::Rgb(190, 190, 190)).add_modifier(Modifier::BOLD))
    }
}

fn create_gap_span() -> Span<'static> {
    Span::raw(" | ")
}
