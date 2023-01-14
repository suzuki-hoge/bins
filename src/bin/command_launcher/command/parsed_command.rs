use bins::libs::item::previewable_item::PreviewableItem;

#[derive(Debug, Eq, PartialEq)]
pub struct ParsedCommand {
    contents: Vec<ParsedContent>,
}

impl ParsedCommand {
    pub fn new(contents: Vec<ParsedContent>) -> Self {
        ParsedCommand { contents }
    }

    pub fn empty() -> Self {
        ParsedCommand { contents: vec![] }
    }

    pub fn get_items(&self) -> Vec<ParsedContent> {
        self.contents.clone()
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct ParsedContent {
    key: String,
    bodies: Vec<String>,
}

impl ParsedContent {
    pub fn new(key: String, bodies: Vec<String>) -> Self {
        ParsedContent { key, bodies }
    }
}

impl PreviewableItem for ParsedContent {
    fn get_origin(&self) -> String {
        self.key.clone()
    }

    fn get_preview(&self) -> Vec<String> {
        self.bodies.clone()
    }
}
