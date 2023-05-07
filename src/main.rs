use std::env;
use tui::layout::{Constraint, Direction};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::ListItem;

use bins::fuzzy::app::process;
use bins::fuzzy::command::CommandType::{GuideSwitch, HorizontalMove, Input, MultiSelect, TabSwitch, VerticalMove};
use bins::fuzzy::core::guide::Guide;
use bins::fuzzy::core::item::Item;
use bins::fuzzy::core::tab::{Tab, TabNames};
use bins::fuzzy::state::State;
use bins::fuzzy::view::{PanesView, SimpleView, TabView};

#[derive(Clone, Debug)]
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

    fn tab_filter(&self, tab: &Tab) -> bool {
        match tab.current {
            0 => true,
            1 => self.line.contains('1'),
            2 => self.line.contains('2'),
            _ => panic!(),
        }
    }
}

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let arg: &str = args[1].as_ref();

    // todo: capsule
    let items = vec!["command", "core", "state", "view"]
        .repeat(30)
        .into_iter()
        .enumerate()
        .map(|(i, s)| FooItem::new(&format!("{s} {i}")))
        .collect();

    let guide = Guide::new(vec!["edit", "run"]);

    let x = match arg {
        "s" => {
            let view = SimpleView::init();
            let state = State::new(items).guide(guide);
            let command_types = [Input, HorizontalMove, VerticalMove, MultiSelect];

            process(view, state, &command_types)?
        }
        "p" => {
            let view = PanesView::new(Direction::Horizontal, Constraint::Percentage(30));
            let state = State::new(items).guide(guide);
            let command_types = [Input, HorizontalMove, VerticalMove, MultiSelect, GuideSwitch];

            process(view, state, &command_types)?
        }
        "t" => {
            let tab_names = TabNames::new(vec!["All", "Filter-1", "Filter-2"]);
            let state = State::new(items).tab(&tab_names).guide(guide);
            let view = TabView::new(tab_names);
            let command_types = [Input, HorizontalMove, VerticalMove, TabSwitch, GuideSwitch];

            process(view, state, &command_types)?
        }
        _ => panic!(),
    };

    dbg!(x);

    Ok(())
}
