use bins::fuzzy::app::process;
use bins::fuzzy::command::CommandType::{HorizontalMove, Input, MultiSelect, VerticalMove};
use bins::fuzzy::item::Item;
use bins::fuzzy::state::State;
use bins::fuzzy::view::PanesView;
use tui::layout::{Constraint, Direction};

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

    fn get_preview(&self) -> String {
        self.line.to_ascii_uppercase()
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

    process(view, state, &command_types)
}
