use crate::libs::app::input_app::InputApp;
use crate::libs::app::scrolling_select_app::ScrollingSelectApp;
use crate::libs::item::previewable_item::PreviewableItem;
use crate::libs::matcher::string_matcher::Mode;
use itertools::Itertools;

#[derive(Debug)]
pub struct MultiFixApp<Item>
where
    Item: PreviewableItem,
{
    pub input_app: InputApp,
    pub scrolling_select_app: ScrollingSelectApp<Item>,
    fixed_items: Vec<Item>,
}

impl<Item> MultiFixApp<Item>
where
    Item: PreviewableItem,
{
    pub fn init(items: Vec<Item>, per_page: u16, mode: Mode) -> Self {
        let input_app = InputApp::init();
        Self {
            input_app,
            scrolling_select_app: ScrollingSelectApp::init(items, per_page as usize, mode),
            fixed_items: vec![],
        }
    }

    pub fn finish(&self) -> Vec<Item> {
        self.fixed_items.clone()
    }

    pub fn refresh(&mut self) {
        self.scrolling_select_app.refresh(&self.input_app.input.split(' ').collect_vec());
    }

    pub fn fix(&mut self) {
        self.scrolling_select_app.pop_item().iter().for_each(|item| self.fixed_items.push(item.get_origin_item()));
    }
}
