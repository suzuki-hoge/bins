use tui::layout::{Constraint, Direction};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::ListItem;

use bins::fuzzy::app::process;
use bins::fuzzy::command::CommandType::{GuideSwitch, HorizontalMove, Input, TabSwitch, VerticalMove};
use bins::fuzzy::core::guide::Guide;
use bins::fuzzy::core::item::Item;
use bins::fuzzy::core::tab::TabNames;
use bins::fuzzy::state::State;
use bins::fuzzy::view::PanesView;

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

    fn custom_preview_style(&self, s: String) -> ListItem {
        if s.starts_with('+') {
            ListItem::new(Span::styled(s, Style::default().fg(Color::Green)))
        } else if s.starts_with('-') {
            ListItem::new(Span::styled(s, Style::default().fg(Color::Red)))
        } else {
            ListItem::new(Span::from(s))
        }
    }
}

fn main() -> anyhow::Result<()> {
    let items = vec![FooItem::new("command"), FooItem::new("core"), FooItem::new("state"), FooItem::new("view")];
    let tab_names = TabNames::new(vec!["Tab1", "Tab2", "Tab3"]);
    let guide = Guide::new(vec!["Edit", "Run"]);

    let command_types = [Input, HorizontalMove, VerticalMove, TabSwitch, GuideSwitch];

    let state = State::new(items).tab(&tab_names).guide(guide);

    // let view = TabView::new(tab_names);
    // let view = SimpleView::init();
    let view = PanesView::new(Direction::Horizontal, Constraint::Percentage(30));

    process(view, state, &command_types)?;

    Ok(())
}
