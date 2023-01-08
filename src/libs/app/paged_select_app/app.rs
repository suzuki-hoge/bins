use std::collections::HashMap;
use std::fmt::Debug;

use crate::libs::app::paged_select_app::matched::Matched;
use crate::libs::app::paged_select_app::page::Page;
use itertools::Itertools;

#[derive(Debug)]
pub struct PagedSelectApp<Item, MatchedItem> {
    all_items: HashMap<usize, Item>,
    matched: Matched<MatchedItem>,
    page: Page,
}

impl<Item, MatchedItem> PagedSelectApp<Item, MatchedItem> {
    pub fn init<Matcher>(items: Vec<Item>, matcher: Matcher, per_page: usize) -> Self
    where
        Matcher: Fn(&Item) -> Option<MatchedItem>,
    {
        let all_items = items.into_iter().enumerate().collect();
        let mut s = Self { all_items, matched: Matched::init(), page: Page::init(per_page) };
        s.matched.refresh(&s.all_items, matcher);
        s.page.refresh2(s.matched.get_item_numbers());
        s
    }

    pub fn get_matched_items_in_page(&self) -> Vec<(&usize, &MatchedItem)> {
        self.page.get_item_numbers_in_page().iter().map(|item_number| self.matched.get_item(item_number)).collect_vec()
    }

    pub fn get_active_item(&self) -> &Item {
        &self.all_items[self.page.get_active_item_number()]
    }

    pub fn is_active_item_number(&self, item_number: &usize) -> bool {
        self.page.get_active_item_number() == item_number
    }

    pub fn up(&mut self) {
        if !self.matched.is_empty() && !self.is_head_of_all() {
            self.page.up(self.matched.get_item_numbers());
        }
    }

    pub fn down(&mut self) {
        if !self.matched.is_empty() && !self.is_last_of_all() {
            self.page.down(self.matched.get_item_numbers());
        }
    }

    pub fn refresh<Matcher>(&mut self, matcher: Matcher)
    where
        Matcher: Fn(&Item) -> Option<MatchedItem>,
    {
        self.matched.refresh(&self.all_items, matcher);
        self.page.page_top(self.matched.get_item_numbers());
    }

    pub fn pop_item(&mut self) -> Option<Item> {
        if self.matched.is_empty() {
            return None;
        }
        let is_last_one_of_matched = self.matched.is_last_number(self.page.get_active_item_number());

        self.matched.remove(self.page.get_active_item_number());

        self.page.refresh2(self.matched.get_item_numbers());

        let item = self.all_items.remove(self.page.get_active_item_number()).unwrap();

        if self.matched.is_empty() {
            return Some(item);
        } else if is_last_one_of_matched {
            self.page.up(self.matched.get_item_numbers());
        } else {
            self.page.refresh2(self.matched.get_item_numbers());
        }

        Some(item)
    }

    fn is_head_of_all(&self) -> bool {
        self.matched.is_head_number(self.page.get_active_item_number())
    }

    fn is_last_of_all(&self) -> bool {
        self.matched.is_last_number(self.page.get_active_item_number())
    }
}
