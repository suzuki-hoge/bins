use itertools::Itertools;

use crate::libs::app::input_app::InputApp;
use crate::libs::app::multi_fix_app::CursorMode::{Edit, Filter};
use crate::libs::app::scrolling_select_app::ScrollingSelectApp;
use crate::libs::item::display_item::DisplayItem;
use crate::libs::matcher::string_matcher::MatchMode;

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum CursorMode {
    Filter,
    Edit,
}

#[derive(Debug)]
pub struct MultiFixApp<Item>
where
    Item: DisplayItem,
{
    pub filter_input_app: InputApp,
    pub scrolling_select_app: ScrollingSelectApp<Item>,
    fixed_items: Vec<Item>,
    pub cursor_mode: CursorMode,
    pub edit_input_app: InputApp,
}

impl<Item> MultiFixApp<Item>
where
    Item: DisplayItem,
{
    pub fn init(items: Vec<Item>, per_page: u16, match_mode: MatchMode) -> Self {
        let filter_input_app = InputApp::init();
        let edit_input_app = InputApp::init();
        Self {
            filter_input_app,
            scrolling_select_app: ScrollingSelectApp::init(items, per_page as usize, match_mode),
            fixed_items: vec![],
            cursor_mode: Filter,
            edit_input_app,
        }
    }

    pub fn is_filter_mode(&self) -> bool {
        self.cursor_mode == Filter
    }

    pub fn is_edit_mode(&self) -> bool {
        !self.is_filter_mode()
    }

    pub fn finish(&self) -> Vec<Item> {
        self.fixed_items.clone()
    }

    pub fn switch_cursor_mode(&mut self) -> bool {
        match self.cursor_mode {
            Filter => {
                let item = self.scrolling_select_app.get_active_item().unwrap();
                if !item.is_editable() {
                    false
                } else {
                    let lines = item.get_pane2();
                    self.edit_input_app.set(lines);
                    self.cursor_mode = Edit;
                    true
                }
            }
            Edit => {
                self.cursor_mode = Filter;
                true
            }
        }
    }

    pub fn change_to_filter_mode(&mut self) {
        self.cursor_mode = Filter;
    }

    pub fn insert(&mut self, c: char) {
        match self.cursor_mode {
            Filter => {
                self.filter_input_app.insert(c);
                self.refresh();
            }
            Edit => {
                self.edit_input_app.insert(c);
            }
        }
    }

    pub fn remove(&mut self) {
        match self.cursor_mode {
            Filter => {
                self.filter_input_app.remove();
                self.refresh();
            }
            Edit => {
                self.edit_input_app.remove();
            }
        }
    }

    pub fn cut(&mut self) {
        match self.cursor_mode {
            Filter => {
                self.filter_input_app.cut();
                self.refresh();
            }
            Edit => {
                self.edit_input_app.cut();
            }
        }
    }

    fn refresh(&mut self) {
        self.scrolling_select_app.refresh(&self.filter_input_app.input.split(' ').collect_vec());
    }

    pub fn fix(&mut self) {
        self.scrolling_select_app.pop_item().iter().for_each(|item| self.fixed_items.push(item.get_origin_item()));
    }
}
