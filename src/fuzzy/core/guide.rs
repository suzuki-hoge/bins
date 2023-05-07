#[derive(Debug)]
pub struct Guide {
    pub labels: Vec<Label>,
}

impl Guide {
    pub fn new(labels: Vec<&'static str>) -> Self {
        Self { labels: labels.into_iter().map(Label::new).collect() }
    }
}

#[derive(Debug)]
pub struct Label {
    pub c: char,
    pub value: &'static str,
}

impl Label {
    fn new(value: &'static str) -> Self {
        Self { c: value.chars().next().unwrap().to_ascii_uppercase(), value }
    }
}
