use bins::libs::item::display_item::DisplayItem;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Command {
    label: String,
    lines: Vec<String>,
}

impl Command {
    pub fn new(label: String, lines: Vec<String>) -> Self {
        Command { label, lines }
    }
}

impl DisplayItem for Command {
    fn get_pane1(&self) -> String {
        self.label.clone()
    }

    fn get_pane2(&self) -> Vec<String> {
        self.lines.clone()
    }
}
