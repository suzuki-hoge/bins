use std::collections::HashMap;
use std::fmt::Debug;

use itertools::Itertools;

#[derive(Debug)]
pub struct ScrollingSelectApp<Item, MatchedItem> {
    all_items: HashMap<usize, Item>,

    matched_items: HashMap<usize, MatchedItem>,
    matched_item_numbers: Vec<usize>,

    head_index_in_page: usize,
    last_index_in_page: usize,
    active_item_index: usize,
}

impl<Item, MatchedItem> ScrollingSelectApp<Item, MatchedItem> {
    pub fn init<Matcher>(items: Vec<Item>, matcher: Matcher, per_page: usize) -> Self
    where
        Matcher: Fn(&Item) -> Option<MatchedItem>,
    {
        let all_items = items.into_iter().enumerate().collect();
        let mut s = Self {
            all_items,
            matched_items: HashMap::new(),
            matched_item_numbers: vec![],
            head_index_in_page: 0,
            last_index_in_page: per_page,
            active_item_index: 0,
        };
        s.re_match(matcher);
        s
    }

    pub fn get_matched_items_in_page(&self) -> Vec<(&usize, &MatchedItem)> {
        self.matched_item_numbers
            .iter()
            .enumerate()
            .filter(|&(item_index, _)| self.head_index_in_page <= item_index && item_index <= self.last_index_in_page)
            .map(|(_, item_number)| self.get_matched_item(item_number))
            .collect_vec()
    }

    pub fn is_active_item_number(&self, item_number: &usize) -> bool {
        self.get_active_item_number() == Some(*item_number)
    }

    pub fn up(&mut self) {
        if self.matched_item_numbers.first() != self.get_active_item_number().as_ref() {
            if self.active_item_index == self.head_index_in_page {
                self.head_index_in_page -= 1;
                self.last_index_in_page -= 1;
            }
            self.active_item_index -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.matched_item_numbers.last() != self.get_active_item_number().as_ref() {
            if self.active_item_index == self.last_index_in_page - 1 {
                self.head_index_in_page += 1;
                self.last_index_in_page += 1;
            }
            self.active_item_index += 1;
        }
    }

    pub fn refresh<Matcher>(&mut self, matcher: Matcher)
    where
        Matcher: Fn(&Item) -> Option<MatchedItem>,
    {
        self.re_match(matcher);

        self.head_index_in_page -= self.head_index_in_page;
        self.last_index_in_page -= self.head_index_in_page;
        self.active_item_index = 0;
    }

    pub fn pop_item(&mut self) -> Option<Item> {
        if let Some(active_item_number) = self.get_active_item_number() {
            let item = self.all_items.remove(&active_item_number).unwrap();
            self.matched_item_numbers.retain(|&item_number| item_number != active_item_number);
            self.matched_items.remove(&active_item_number);

            if self.matched_item_numbers.is_empty() {
                return Some(item);
            } else if self.active_item_index != 0
                && self.matched_item_numbers.last() == self.matched_item_numbers.get(self.active_item_index - 1)
            {
                self.up();
            } else {
                // do nothing
            }

            Some(item)
        } else {
            None
        }
    }

    // match

    fn get_matched_item<'a>(&'a self, target_item_number: &'a usize) -> (&'a usize, &'a MatchedItem) {
        (target_item_number, self.matched_items.get(target_item_number).unwrap())
    }

    fn re_match<Matcher>(&mut self, matcher: Matcher)
    where
        Matcher: Fn(&Item) -> Option<MatchedItem>,
    {
        self.matched_item_numbers = vec![];
        self.matched_items = HashMap::new();

        for item_number in self.all_items.keys() {
            let matched_item = matcher(self.all_items.get(item_number).unwrap());

            matched_item.into_iter().for_each(|item| {
                self.matched_item_numbers.push(*item_number);
                self.matched_items.insert(*item_number, item);
            })
        }
        self.matched_item_numbers.sort_unstable();
    }

    // page

    fn get_active_item_number(&self) -> Option<usize> {
        self.matched_item_numbers.get(self.active_item_index).copied()
    }
}
