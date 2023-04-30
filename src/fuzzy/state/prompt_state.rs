#[derive(Debug)]
pub struct PromptState {
    pub input: String,
    pub position: usize,
}

impl PromptState {
    pub fn new() -> Self {
        Self { input: String::new(), position: 0 }
    }
}
