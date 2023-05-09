use std::env;
use tui::layout::{Constraint, Direction};
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::ListItem;

use bins::fuzzy::core::item::Item;
use bins::fuzzy::core::style::CustomPreviewStyle;
use bins::fuzzy::core::tab::Tab;

use bins::fuzzy::FuzzyBuilder;

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

    fn tab_filter(&self, tab: &Tab) -> bool {
        match tab.current {
            0 => true,
            1 => self.line.contains('1'),
            2 => self.line.contains('2'),
            _ => panic!(),
        }
    }
}

struct Diff {}

impl CustomPreviewStyle for Diff {
    fn to_list_item(&self, s: String) -> ListItem {
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
    let args: Vec<String> = env::args().collect();
    let arg: &str = args[1].as_ref();

    let items = vec!["command", "core", "state", "view"]
        .repeat(10000)
        .into_iter()
        .enumerate()
        .map(|(i, s)| FooItem::new(&format!("{s} {i}")))
        .collect();

    let x = match arg {
        "s" => FuzzyBuilder::simple(items).build().run()?,
        "p" => {
            FuzzyBuilder::pane(items, Direction::Horizontal, Constraint::Percentage(30))
                // .custom_preview(Diff{})
                .default_preview()
                .guide(vec!["edit", "run"])
                .build()
                .run()?
        }
        "t" => {
            FuzzyBuilder::tab(items, vec!["All", "Filter-1", "Filter-2"]).guide(vec!["edit", "run"]).build().run()?
        }
        _ => panic!(),
    };

    dbg!(x);

    Ok(())
}
