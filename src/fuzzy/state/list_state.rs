use std::collections::HashSet;

use itertools::Itertools;

use crate::fuzzy::core::item::Item;
use crate::fuzzy::core::tab::Tab;
use crate::fuzzy::matcher::Matcher;
use crate::fuzzy::state::list_state::LineStatus::{Active, ActiveSelected, Normal, Selected};

pub enum LineStatus {
    Active,
    Selected,
    ActiveSelected,
    Normal,
}

#[derive(Debug)]
pub struct ListState<I: Item> {
    items: Vec<I>,
    matcher: Matcher,
    active_line_number: usize,
    matched_ids: Vec<usize>,
    selected_ids: HashSet<usize>,
}

impl<I: Item> ListState<I> {
    pub fn new(items: Vec<I>) -> Self {
        Self {
            items,
            matcher: Matcher::new(""),
            active_line_number: 0,
            matched_ids: vec![],
            selected_ids: HashSet::new(),
        }
    }

    pub fn up(&mut self) {
        if self.active_line_number == 0 {
            self.active_line_number = self.matched_ids.len() - 1;
        } else {
            self.active_line_number -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.active_line_number == self.matched_ids.len() - 1 {
            self.active_line_number = 0;
        } else {
            self.active_line_number += 1;
        }
    }

    pub fn select(&mut self) {
        let id = self.matched_ids[self.active_line_number];
        if self.selected_ids.contains(&id) {
            self.selected_ids.remove(&id);
        } else {
            self.selected_ids.insert(id);
        }
    }

    pub fn fix(&mut self) {
        let id = self.matched_ids[self.active_line_number];
        self.selected_ids.insert(id);
    }

    pub fn get_active_item(&self) -> Option<&I> {
        if self.active_line_number < self.matched_ids.len() {
            Some(&self.items[self.matched_ids[self.active_line_number]])
        } else {
            None
        }
    }

    pub fn rematch(&mut self, input: &str, preview: bool, tab: Option<&Tab>) {
        self.matcher = Matcher::new(input);

        self.active_line_number = 0;

        self.matched_ids = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                let tab_matched = tab.map(|t| item.tab_filter(t)).unwrap_or(true);
                let item_matched = if preview {
                    self.matcher.is_match(&item.get_line()) || self.matcher.is_match(&item.get_preview().join("\n"))
                } else {
                    self.matcher.is_match(&item.get_line())
                };
                item_matched && tab_matched
            })
            .map(|(id, _)| id)
            .collect();
    }

    pub fn get_matched_line_parts(&self, page_size: u16) -> Vec<(LineStatus, Vec<(String, bool)>)> {
        let (page_range_s, page_range_e) = get_page_range(self.active_line_number, page_size);

        self.matched_ids
            .iter()
            .enumerate()
            .filter(|(line_number, _)| page_range_s <= *line_number && *line_number <= page_range_e)
            .map(|(line_number, &id)| {
                let status = if line_number == self.active_line_number && self.selected_ids.contains(&id) {
                    ActiveSelected
                } else if line_number == self.active_line_number {
                    Active
                } else if self.selected_ids.contains(&id) {
                    Selected
                } else {
                    Normal
                };
                (status, self.matcher.get_matched_parts(&self.items[id].get_line()))
            })
            .collect()
    }

    pub fn get_simple_preview(&self) -> Vec<String> {
        if let Some(item) = self.get_active_item() {
            item.get_preview()
        } else {
            vec![]
        }
    }

    pub fn get_matched_preview_parts(&self) -> Vec<Vec<(String, bool)>> {
        if let Some(item) = self.get_active_item() {
            item.get_preview().iter().map(|line| self.matcher.get_matched_parts(line)).collect()
        } else {
            vec![]
        }
    }

    pub fn get_selected_items(&self) -> Vec<I> {
        let mut ids = self.selected_ids.iter().collect_vec();
        ids.sort();
        ids.into_iter().map(|&id| self.items[id].clone()).collect()
    }
}

fn get_page_range(line_number: usize, page_size: u16) -> (usize, usize) {
    let page_size = page_size as usize;
    let s = line_number / page_size * page_size;
    (s, s + page_size - 1)
}

#[cfg(test)]
mod tests {
    use crate::fuzzy::state::list_state::get_page_range;

    #[test]
    fn range_test() {
        assert_eq!(get_page_range(0, 4), (0, 3));
        assert_eq!(get_page_range(1, 4), (0, 3));
        assert_eq!(get_page_range(2, 4), (0, 3));
        assert_eq!(get_page_range(3, 4), (0, 3));

        assert_eq!(get_page_range(4, 4), (4, 7));
        assert_eq!(get_page_range(5, 4), (4, 7));
        assert_eq!(get_page_range(6, 4), (4, 7));
        assert_eq!(get_page_range(7, 4), (4, 7));
    }
}
