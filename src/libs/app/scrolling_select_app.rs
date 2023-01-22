use std::collections::HashMap;
use std::fmt::Debug;

use crate::libs::matcher::string_matcher::{CheckedString, Mode};

use crate::libs::item::display_item::DisplayItem;
use itertools::Itertools;
use rayon::prelude::*;

// xxx_number: index for all_items ( means item id )
// xxx_index: index for page

#[derive(Debug)]
pub struct ScrollingSelectApp<Item>
where
    Item: DisplayItem,
{
    all_items: HashMap<usize, CheckedString<Item>>,
    matched_items: HashMap<usize, CheckedString<Item>>,
    matched_item_numbers: Vec<usize>,
    head_index_in_page: usize,
    last_index_in_page: usize,
    active_item_index: usize,
    mode: Mode,
}

impl<Item> ScrollingSelectApp<Item>
where
    Item: DisplayItem,
{
    pub fn init(items: Vec<Item>, per_page: usize, mode: Mode) -> Self {
        let all_items = items
            .into_par_iter()
            .enumerate()
            .map(|(item_number, item)| (item_number, CheckedString::init(item)))
            .collect();

        let mut s = Self {
            all_items,
            matched_items: HashMap::new(),
            matched_item_numbers: vec![],
            head_index_in_page: 0,
            last_index_in_page: per_page,
            active_item_index: 0,
            mode,
        };
        s.re_match(&[]);
        s
    }

    pub fn get_matched_items_in_page(&self) -> Vec<(&usize, &CheckedString<Item>)> {
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

    pub fn refresh(&mut self, words: &[&str]) {
        self.re_match(words);

        self.head_index_in_page -= self.head_index_in_page;
        self.last_index_in_page -= self.head_index_in_page;
        self.active_item_index = 0;
    }

    pub fn get_active_item(&self) -> Option<CheckedString<Item>> {
        if let Some(active_item_number) = self.get_active_item_number() {
            self.matched_items.get(&active_item_number).cloned()
        } else {
            None
        }
    }

    pub fn pop_item(&mut self) -> Option<CheckedString<Item>> {
        if let Some(active_item_number) = self.get_active_item_number() {
            self.all_items.remove(&active_item_number);
            self.matched_item_numbers.retain(|&item_index| item_index != active_item_number);
            let item = self.matched_items.remove(&active_item_number);

            if self.matched_item_numbers.is_empty() {
                return item;
            } else if self.active_item_index != 0
                && self.matched_item_numbers.last() == self.matched_item_numbers.get(self.active_item_index - 1)
            {
                self.up();
            } else {
                // do nothing
            }

            item
        } else {
            None
        }
    }

    // match

    fn get_matched_item<'a>(&'a self, target_item_number: &'a usize) -> (&'a usize, &'a CheckedString<Item>) {
        (target_item_number, self.matched_items.get(target_item_number).unwrap())
    }

    fn re_match(&mut self, words: &[&str]) {
        let lower_words = words.iter().map(|word| word.to_lowercase()).filter(|word| !word.is_empty()).collect_vec();

        self.matched_items = HashMap::new();
        self.matched_item_numbers = vec![];

        let matched_items_with_number: Vec<(usize, CheckedString<Item>)> = self
            .all_items
            .par_iter()
            .flat_map(|(item_number, item)| {
                let checked_item = item.clone().re_match(&lower_words, self.mode);
                if checked_item.is_matched(self.mode) {
                    vec![(*item_number, checked_item)]
                } else {
                    vec![]
                }
            })
            .collect();

        let matched_item_numbers = matched_items_with_number.par_iter().map(|(item_number, _)| *item_number).collect();
        self.matched_item_numbers = matched_item_numbers;
        self.matched_item_numbers.sort_unstable();

        let matched_items = matched_items_with_number.into_par_iter().map(|(a, b)| (a, b)).collect();
        self.matched_items = matched_items;

        self.matched_item_numbers.sort_unstable();
    }

    // page

    fn get_active_item_number(&self) -> Option<usize> {
        self.matched_item_numbers.get(self.active_item_index).copied()
    }
}
