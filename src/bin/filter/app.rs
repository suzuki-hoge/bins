use bins::apps::input_app::InputApp;
use bins::apps::matched_string::MatchedString;

#[derive(Debug)]
pub struct App {
    pub input: InputApp,
    pub origin_lines: Vec<String>,
    pub matched_lines: Vec<Option<MatchedString>>, // same length as origin_lines
    pub fixed_lines: Vec<String>,
    pub cursor: usize,
}

impl App {
    pub fn init(lines: Vec<String>) -> Self {
        Self {
            input: InputApp::init(),
            origin_lines: lines.clone(),
            matched_lines: lines
                .iter()
                .map(|line| MatchedString::matched_only("", line))
                .collect(),
            fixed_lines: vec![],
            cursor: 0,
        }
    }

    // cursor

    pub fn down(&mut self) {
        if self.cursor != self.matched_lines.len() - 1 {
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
        self.matched_lines = self
            .origin_lines
            .iter()
            .map(|line| MatchedString::matched_only(&self.input.input, line))
            .collect();
        self.cursor = 0;
    }

    pub fn fix(&mut self) {
        let index = self.find_index();

        let line = self.matched_lines[index]
            .as_ref()
            .unwrap()
            .chars
            .iter()
            .fold("".to_string(), |acc, mc| format!("{}{}", acc, mc.value));
        self.fixed_lines.push(line);
        self.origin_lines.remove(index);
        self.matched_lines.remove(index);

        if self.cursor == self.matched_lines.len() {
            self.up();
        }
    }

    fn find_index(&self) -> usize {
        let mut found = 0;
        for i in 0..self.matched_lines.len() {
            if self.matched_lines[i].is_some() {
                found += 1;
            }
            if found == self.cursor + 1 {
                return i;
            }
        }
        panic!("n/a");
    }

    pub fn finish(self) -> Vec<String> {
        self.fixed_lines
    }
}

#[cfg(test)]
mod tests {
    use crate::app::App;

    #[test]
    fn fix() {
        let mut app = App::init(
            vec!["youtube", "github", "instagram", "twitter"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        // input char

        app.input.insert('e');
        app.refresh();

        assert!(app.matched_lines[0].is_some());
        assert!(app.matched_lines[1].is_none());
        assert!(app.matched_lines[2].is_none());
        assert!(app.matched_lines[3].is_some());

        // fix

        app.down();
        app.fix();

        assert!(app.matched_lines[0].is_some());
        assert!(app.matched_lines[1].is_none());
        assert!(app.matched_lines[2].is_none());
        assert_eq!(app.fixed_lines, vec!["twitter"]);

        // delete

        app.input.remove();
        app.refresh();

        // fix

        app.fix();

        assert!(app.matched_lines[0].is_some());
        assert!(app.matched_lines[1].is_some());
        assert_eq!(app.fixed_lines, vec!["twitter", "youtube"]);
    }
}
