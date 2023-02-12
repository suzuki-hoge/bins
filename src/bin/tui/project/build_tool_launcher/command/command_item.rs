use bins::libs::item::display_item::DisplayItem;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct CommandItem {
    label: String,
    lines: Vec<String>,
    is_editable: bool,
}

impl CommandItem {
    pub fn new(label: String, lines: Vec<String>) -> Self {
        Self { label, lines, is_editable: false }
    }

    pub fn new_editable(label: String, lines: Vec<String>) -> Self {
        Self { label, lines, is_editable: true }
    }

    pub fn get_runnable(&self) -> String {
        match self.is_editable {
            true => self.get_pane2().join("\n"),
            false => self.get_pane1(),
        }
    }
}

impl DisplayItem for CommandItem {
    fn get_pane1(&self) -> String {
        self.label.clone()
    }

    fn get_pane2(&self) -> Vec<String> {
        self.lines.clone()
    }

    fn is_editable(&self) -> bool {
        self.is_editable
    }
}
