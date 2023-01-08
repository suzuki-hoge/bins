use bins::libs::app::input_app::InputApp;
use bins::libs::app::scrolling_select_app::ScrollingSelectApp;

use itertools::Itertools;

#[derive(Debug)]
pub struct App {
    pub input_app: InputApp,
    pub scrolling_select_app: ScrollingSelectApp,
    fixed_strings: Vec<String>,
}

impl App {
    pub fn init(items: Vec<String>, per_page: u16) -> Self {
        let input_app = InputApp::init();
        Self {
            input_app,
            scrolling_select_app: ScrollingSelectApp::init(items, per_page as usize),
            fixed_strings: vec![],
        }
    }

    pub fn finish(self) -> Vec<String> {
        self.fixed_strings
    }

    pub fn refresh(&mut self) {
        self.scrolling_select_app.refresh(&self.input_app.input.split(' ').collect_vec());
    }

    pub fn fix(&mut self) {
        self.scrolling_select_app.pop_item().iter().for_each(|item| self.fixed_strings.push(item.get_origin()));
    }
}
