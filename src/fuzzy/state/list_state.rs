use crate::fuzzy::core::item::Item;
use crate::fuzzy::core::tab::Tab;
use crate::fuzzy::matcher::Matcher;

#[derive(Debug)]
pub struct ListState<I: Item> {
    items: Vec<I>,
    matcher: Matcher,
    active_line_number: usize,
    matched_indices: Vec<usize>,
}

impl<I: Item> ListState<I> {
    pub fn new(items: Vec<I>) -> Self {
        Self { items, matcher: Matcher::new(""), active_line_number: 0, matched_indices: vec![] }
    }

    pub fn up(&mut self) {
        self.active_line_number -= 1;
    }

    pub fn down(&mut self) {
        self.active_line_number += 1;
    }

    pub fn rematch(&mut self, input: &str, tab: Option<&Tab>) {
        self.matcher = Matcher::new(input);

        self.active_line_number = 0;

        self.matched_indices = self
            .items
            .iter()
            .enumerate()
            .filter(|(_, item)| {
                self.matcher.is_match(&item.get_line()) && tab.map(|t| item.tab_filter(t)).unwrap_or(true)
            })
            .map(|(i, _)| i)
            .collect();
    }

    pub fn get_matched_lines(&self, _page_size: u16) -> Vec<String> {
        self.matched_indices
            .iter()
            .enumerate()
            .map(|(line_number, &index)| {
                format!(
                    "{} {}: {}",
                    if line_number == self.active_line_number { "*" } else { " " },
                    index,
                    self.items[index].get_line()
                )
            })
            .collect()
    }

    pub fn get_matched_line_parts(&self) -> Vec<(bool, Vec<(String, bool)>)> {
        self.matched_indices
            .iter()
            .enumerate()
            .map(|(line_number, &index)| {
                (line_number == self.active_line_number, self.matcher.get_matched_parts(&self.items[index].get_line()))
            })
            .collect()
    }

    pub fn get_active_item(&self) -> Option<&I> {
        if self.active_line_number < self.matched_indices.len() {
            Some(&self.items[self.matched_indices[self.active_line_number]])
        } else {
            None
        }
    }
}
