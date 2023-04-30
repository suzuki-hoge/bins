use std::fs::File;

use crate::fuzzy::state::prompt_state::PromptState;
use tui::backend::CrosstermBackend;
use tui::layout::Rect;
use tui::widgets::Paragraph;
use tui::Frame;

const PROMPT: &str = "> ";

pub fn render_prompt(frame: &mut Frame<CrosstermBackend<File>>, rect: Rect, state: &PromptState) {
    frame.set_cursor(frame.size().x + (PROMPT.len() + state.position) as u16, frame.size().y);

    let paragraph = Paragraph::new(format!("{}{}", PROMPT, state.input));

    frame.render_widget(paragraph, rect);
}
