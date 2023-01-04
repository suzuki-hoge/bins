use std::collections::HashMap;
use std::fmt::Debug;

use crate::libs::app::paged_select_app::active::Active;
use crate::libs::app::paged_select_app::matched::Matched;
use crate::libs::app::paged_select_app::page::Page;
use itertools::Itertools;

#[derive(Debug)]
pub struct PagedSelectApp<Item, MatchedItem> {
    all_items: HashMap<usize, Item>,
    matched: Matched<MatchedItem>,
    active: Active,
    page: Page,
}

impl<Item, MatchedItem> PagedSelectApp<Item, MatchedItem> {
    pub fn init<Matcher>(items: Vec<Item>, matcher: Matcher) -> Self
    where
        Matcher: Fn(&Item) -> Option<MatchedItem>,
    {
        let all_items = items.into_iter().enumerate().collect();
        let mut s = Self { all_items, matched: Matched::init(), active: Active::init(), page: Page::init() };
        s.matched.refresh(&s.all_items, matcher);
        s.page.refresh(s.matched.get_item_numbers());
        s
    }

    pub fn get_matched_items_in_page(&self) -> Vec<(&usize, &MatchedItem)> {
        self.page.get_item_numbers_in_page().iter().map(|item_number| self.matched.get_item(item_number)).collect_vec()
    }

    pub fn get_item(&self) -> &Item {
        &self.all_items[self.active.get_item_number()]
    }

    pub fn is_active_item_number(&self, item_number: &usize) -> bool {
        self.active.get_item_number() == item_number
    }

    pub fn up(&mut self) {
        if !self.is_head_of_all() {
            self.active.up(self.matched.get_item_numbers());

            if self.page.is_out_of_page(self.active.get_item_index()) {
                self.page.turn_prev(self.matched.get_item_numbers());
            }
        }
    }

    pub fn down(&mut self) {
        if !self.is_last_of_all() {
            self.active.down(self.matched.get_item_numbers());

            if self.page.is_out_of_page(self.active.get_item_index()) {
                self.page.turn_next(self.matched.get_item_numbers());
            }
        }
    }

    pub fn refresh<Matcher>(&mut self, matcher: Matcher)
    where
        Matcher: Fn(&Item) -> Option<MatchedItem>,
    {
        self.matched.refresh(&self.all_items, matcher);
        self.page.turn_top();
        self.page.refresh(self.matched.get_item_numbers());
        self.active.page_top(self.page.get_head_index_in_page(), self.matched.get_item_numbers());
    }

    pub fn pop_item(&mut self) -> Item {
        let is_last_one_of_all = self.matched.is_last_number(self.active.get_item_number());

        self.matched.remove(self.active.get_item_number());

        self.page.refresh(self.matched.get_item_numbers());

        let item = self.all_items.remove(self.active.get_item_number()).unwrap();

        if is_last_one_of_all {
            self.active.up(self.matched.get_item_numbers());
        } else {
            self.active.refresh(self.matched.get_item_numbers());
        }

        item
    }

    fn is_head_of_all(&self) -> bool {
        self.matched.is_head_number(self.active.get_item_number())
    }

    fn is_last_of_all(&self) -> bool {
        self.matched.is_last_number(self.active.get_item_number())
    }
}
