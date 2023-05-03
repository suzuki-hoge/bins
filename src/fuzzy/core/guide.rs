#[derive(Debug)]
pub struct Guide {
    pub labels: Vec<Label>,
}

impl Guide {
    pub fn new(labels: Vec<String>) -> Self {
        Self { labels: labels.into_iter().map(Label::new).collect() }
    }
}

#[derive(Debug)]
pub struct Label {
    pub c: char,
    pub value: String,
}

impl Label {
    fn new(value: String) -> Self {
        Self { c: value.chars().next().unwrap(), value }
    }
}
