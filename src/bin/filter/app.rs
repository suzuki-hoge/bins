use bins::apps::input_app::InputApp;
use bins::apps::matched_string::MatchedString;

#[derive(Debug)]
pub struct App {
    pub input: InputApp,
    pub lines: Vec<String>,
    pub matched_strings: Vec<MatchedString>,
    pub cursor: usize,
}

impl App {
    pub fn init(lines: Vec<String>) -> Self {
        Self {
            input: InputApp::init(),
            lines: lines.clone(),
            matched_strings: lines
                .iter()
                .flat_map(|line| MatchedString::matched_only("", line))
                .collect(),
            cursor: 0,
        }
    }

    // cursor

    pub fn down(&mut self) {
        if self.cursor != self.matched_strings.len() - 1 {
            self.cursor += 1;
        }
    }

    pub fn up(&mut self) {
        if self.cursor != 0 {
            self.cursor -= 1;
        }
    }

    // action

    pub fn refresh(&mut self) {
        self.matched_strings = self
            .lines
            .iter()
            .flat_map(|line| MatchedString::matched_only(&self.input.input, line))
            .collect();
        self.cursor = 0;
    }
}
