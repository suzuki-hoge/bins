use bins::fuzzy::core::item::Item;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct CommandItem {
    label: String,
    lines: Vec<String>,
    is_bb: bool,
}

impl CommandItem {
    pub fn new<S: Into<String>>(label: S, lines: Vec<S>, is_bb: bool) -> Self {
        Self { label: label.into(), lines: lines.into_iter().map(|line| line.into()).collect(), is_bb }
    }

    pub fn as_runnable(&self) -> String {
        match self.is_bb {
            true => self.get_preview().join("\n"),
            false => self.get_line(),
        }
    }

    pub fn is_bb_match(&self, label: &str) -> bool {
        self.is_bb && self.label == format!("bb {label}")
    }
}

impl Item for CommandItem {
    fn get_line(&self) -> String {
        self.label.to_string()
    }

    fn get_preview(&self) -> Vec<String> {
        self.lines.clone()
    }
}
