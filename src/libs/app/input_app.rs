#[derive(Debug)]
pub struct InputApp {
    pub input: String,
    pub cursor: usize,
}

impl InputApp {
    pub fn init() -> Self {
        Self { input: "".to_string(), cursor: 0 }
    }

    // edit

    pub fn insert(&mut self, c: char) {
        self.input.insert(self.cursor, c);
        self.cursor += 1;
    }

    pub fn remove(&mut self) {
        if !self.input.is_empty() {
            self.input.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }

    // action

    pub fn cut(&mut self) {
        self.input = self.input[..self.cursor].to_string();
        self.cursor = self.input.len();
    }

    // cursor

    pub fn right(&mut self) {
        if self.cursor != self.input.len() {
            self.cursor += 1;
        }
    }

    pub fn left(&mut self) {
        if self.cursor != 0 {
            self.cursor -= 1;
        }
    }

    pub fn top(&mut self) {
        self.cursor = 0;
    }

    pub fn end(&mut self) {
        self.cursor = self.input.len();
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::app::input_app::InputApp;

    #[test]
    fn edit() {
        let mut app = InputApp::init();

        // insert chars

        app.insert('o');
        app.insert('k');
        assert_eq!(app.input, "ok");
        assert_eq!(app.cursor, 2);

        // remove char

        app.remove();
        app.remove();
        assert_eq!(app.input, "");
        assert_eq!(app.cursor, 0);

        // guard empty

        app.remove();
        assert_eq!(app.input, "");
        assert_eq!(app.cursor, 0);
    }

    #[test]
    fn cursor() {
        let mut app = InputApp::init();
        app.insert('o');
        app.insert('k');

        // move left

        app.left();
        app.left();
        assert_eq!(app.input, "ok");
        assert_eq!(app.cursor, 0);

        // guard too left

        app.left();
        assert_eq!(app.input, "ok");
        assert_eq!(app.cursor, 0);

        // move right

        app.right();
        app.right();
        assert_eq!(app.input, "ok");
        assert_eq!(app.cursor, 2);

        // guard too right

        app.right();
        assert_eq!(app.input, "ok");
        assert_eq!(app.cursor, 2);

        // move top

        app.top();
        assert_eq!(app.input, "ok");
        assert_eq!(app.cursor, 0);

        // move end

        app.end();
        assert_eq!(app.input, "ok");
        assert_eq!(app.cursor, 2);

        // insert chars

        app.left();
        app.insert('-');
        app.insert('>');
        assert_eq!(app.input, "o->k");
        assert_eq!(app.cursor, 3);
    }

    #[test]
    fn action() {
        let mut app = InputApp::init();
        app.insert('h');
        app.insert('e');
        app.insert('l');
        app.insert('l');
        app.insert('o');

        // cut chars

        app.top();
        app.right();
        app.right();
        app.cut();
        assert_eq!(app.input, "he");
        assert_eq!(app.cursor, 2);
    }
}
