use bins::apps::input_app::InputApp;

#[derive(Debug)]
pub struct App {
    pub input: InputApp,
    pub lines: Vec<String>,
}

impl App {
    pub fn init(lines: Vec<String>) -> Self {
        Self {
            input: InputApp::init(),
            lines,
        }
    }
}
