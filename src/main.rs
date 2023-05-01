use bins::fuzzy::app::process;
use bins::fuzzy::command::CommandType::{HorizontalMove, Input, MultiSelect, VerticalMove};
use bins::fuzzy::item::Item;
use bins::fuzzy::state::State;
use bins::fuzzy::view::PanesView;

use tui::layout::{Constraint, Direction};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::ListItem;

struct FooItem {
    line: String,
}

impl FooItem {
    fn new(s: &str) -> Self {
        Self { line: s.to_string() }
    }
}

impl Item for FooItem {
    fn get_line(&self) -> String {
        self.line.to_string()
    }

    fn get_preview(&self) -> Vec<String> {
        vec![
            String::from("  function foo() {"),
            String::from("+ echo foo"),
            String::from("- echo bar"),
            String::from("  }"),
        ]
    }

    fn custom_preview_style<S: Into<String>>(&self, s: S) -> ListItem {
        let s = s.into();
        if s.starts_with('+') {
            ListItem::new(Span::styled(s, Style::default().fg(Color::Green)))
        } else if s.starts_with('-') {
            ListItem::new(Span::styled(s, Style::default().fg(Color::Red)))
        } else {
            ListItem::new(Span::from(s))
        }
    }

    fn get_tab_names() -> Vec<String> {
        vec![]
    }

    fn shift_tab(&mut self) {
        //
    }
}

fn main() -> anyhow::Result<()> {
    // let view = SimpleView::init();
    let view = PanesView::new(Direction::Horizontal, Constraint::Percentage(30));
    let items = vec![FooItem::new("command"), FooItem::new("item"), FooItem::new("state"), FooItem::new("view")];
    let state = State::new(items);
    let command_types = [Input, HorizontalMove, VerticalMove, MultiSelect];

    process(view, state, &command_types)?;
    Ok(())
}
