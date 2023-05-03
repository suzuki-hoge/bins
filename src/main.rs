use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::ListItem;

use bins::fuzzy::app::process;
use bins::fuzzy::command::CommandType::{GuideSwitch, HorizontalMove, Input, TabSwitch, VerticalMove};
use bins::fuzzy::core::guide::Guide;
use bins::fuzzy::core::item::Item;
use bins::fuzzy::core::tab::{Tab, TabNames};
use bins::fuzzy::state::State;
use bins::fuzzy::view::TabView;

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

    // 'static を返せば？ create_title 参照
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
}

fn main() -> anyhow::Result<()> {
    // let view = SimpleView::init();
    // let view = PanesView::new(Direction::Horizontal, Constraint::Percentage(30));
    let tab_names = TabNames::new(vec![String::from("Tab1"), String::from("Tab2"), String::from("Tab3")]);
    let items = vec![FooItem::new("command"), FooItem::new("core"), FooItem::new("state"), FooItem::new("view")];
    let guide = Guide::new(vec!["Edit".to_string(), "Run".to_string()]);
    let state = State::new(items, Tab::new(&tab_names), guide);
    let view = TabView::new(tab_names);
    let command_types = [Input, HorizontalMove, VerticalMove, TabSwitch, GuideSwitch];

    process(view, state, &command_types)?;
    Ok(())
}
