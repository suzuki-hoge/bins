use bins::libs::app::input_app::InputApp;
use bins::libs::app::scrolling_select_app::ScrollingSelectApp;
use bins::libs::common::matched_string::MatchedString;

#[derive(Debug)]
pub struct App {
    pub input_app: InputApp,
    pub scrolling_select_app: ScrollingSelectApp<String, MatchedString>,
    fixed_items: Vec<String>,
}

impl App {
    pub fn init(items: Vec<String>, per_page: u16) -> Self {
        let input_app = InputApp::init();
        Self {
            input_app,
            scrolling_select_app: ScrollingSelectApp::init(
                items,
                |item| MatchedString::matched_only("", item),
                per_page as usize,
            ),
            fixed_items: vec![],
        }
    }

    pub fn finish(self) -> Vec<String> {
        self.fixed_items
    }

    pub fn refresh(&mut self) {
        self.scrolling_select_app.refresh(|item| MatchedString::matched_only(&self.input_app.input, item));
    }

    pub fn fix(&mut self) {
        self.scrolling_select_app.pop_item().iter().for_each(|item| self.fixed_items.push(item.clone()));
    }
}
