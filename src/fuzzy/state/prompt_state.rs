#[derive(Debug)]
pub struct PromptState {
    pub input: String,
    pub position: usize,
}

impl PromptState {
    pub fn init() -> Self {
        Self { input: String::new(), position: 0 }
    }

    // edit

    pub fn insert(&mut self, c: char) {
        self.input.insert(self.position, c);
        self.position += 1;
    }

    pub fn remove(&mut self) {
        if !self.input.is_empty() {
            self.input.remove(self.position - 1);
            self.position -= 1;
        }
    }

    pub fn cut(&mut self) {
        self.input = self.input[..self.position].to_string();
        self.position = self.input.len();
    }

    // move

    pub fn right(&mut self) {
        if self.position != self.input.len() {
            self.position += 1;
        }
    }

    pub fn left(&mut self) {
        if self.position != 0 {
            self.position -= 1;
        }
    }

    pub fn top(&mut self) {
        self.position = 0;
    }

    pub fn end(&mut self) {
        self.position = self.input.len()
    }
}

#[cfg(test)]
mod tests {
    use crate::fuzzy::state::prompt_state::PromptState;

    #[test]
    fn edit_test() {
        let mut state = PromptState::init();

        // insert chars

        state.insert('o');
        state.insert('k');
        assert_eq!(state.input, "ok");
        assert_eq!(state.position, 2);

        // remove char

        state.remove();
        assert_eq!(state.input, "o");
        assert_eq!(state.position, 1);

        state.remove();
        assert_eq!(state.input, "");
        assert_eq!(state.position, 0);

        // guard empty

        state.remove();
        assert_eq!(state.input, "");
        assert_eq!(state.position, 0);
    }

    #[test]
    fn move_test() {
        let mut state = PromptState::init();
        state.insert('o');
        state.insert('k');

        // move left

        state.left();
        assert_eq!(state.input, "ok");
        assert_eq!(state.position, 1);

        state.left();
        assert_eq!(state.input, "ok");
        assert_eq!(state.position, 0);

        // guard too left

        state.left();
        assert_eq!(state.input, "ok");
        assert_eq!(state.position, 0);

        // move right

        state.right();
        assert_eq!(state.input, "ok");
        assert_eq!(state.position, 1);

        state.right();
        assert_eq!(state.input, "ok");
        assert_eq!(state.position, 2);

        // guard too right

        state.right();
        assert_eq!(state.input, "ok");
        assert_eq!(state.position, 2);

        // move top

        state.top();
        assert_eq!(state.input, "ok");
        assert_eq!(state.position, 0);

        // move end

        state.end();
        assert_eq!(state.input, "ok");
        assert_eq!(state.position, 2);

        // insert chars

        state.left();
        state.insert('-');
        state.insert('>');
        assert_eq!(state.input, "o->k");
        assert_eq!(state.position, 3);
    }

    #[test]
    fn cut_test() {
        let mut state = PromptState::init();
        state.insert('h');
        state.insert('e');
        state.insert('l');
        state.insert('l');
        state.insert('o');

        // cut chars

        state.top();
        state.right();
        state.right();
        state.cut();
        assert_eq!(state.input, "he");
        assert_eq!(state.position, 2);
    }
}
