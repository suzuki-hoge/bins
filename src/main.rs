use bins::fuzzy::app::process;
use bins::fuzzy::command::command::CommandType::{HorizontalMove, Input};
use bins::fuzzy::item::item::Item;
use bins::fuzzy::state::state::State;
use bins::fuzzy::view::view::View;

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
}

fn main() -> anyhow::Result<()> {
    let view = View {};
    let items = vec![FooItem::new("command"), FooItem::new("item"), FooItem::new("state"), FooItem::new("view")];
    let state = State::new(items);
    let command_types = [Input, HorizontalMove];

    process(view, state, &command_types)
}
