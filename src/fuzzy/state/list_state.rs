use crate::fuzzy::core::item::Item;
use crate::fuzzy::core::tab::Tab;

#[derive(Debug)]
pub struct ListState<I: Item> {
    items: Vec<I>,
    active_line_number: usize,
    matched_indices: Vec<usize>,
}

impl<I: Item> ListState<I> {
    pub fn new(items: Vec<I>) -> Self {
        Self { items, active_line_number: 0, matched_indices: vec![] }
    }

    pub fn up(&mut self) {
        self.active_line_number -= 1;
    }

    pub fn down(&mut self) {
        self.active_line_number += 1;
    }

    // tmp
    fn is_match(&self, line: &String, input: &String) -> bool {
        if line.is_empty() {
            true
        } else {
            line.contains(input)
        }
    }

    pub fn rematch(&mut self, input: &String, tab: &Tab) {
        self.active_line_number = 0;
        self.matched_indices = self
            .items
            .iter()
            .enumerate()
            .filter(|(_i, item)| self.is_match(&item.get_line(), input) && item.tab_filter(tab))
            .map(|(i, _)| i)
            .collect();
    }

    pub fn get_matched_lines(&self, page_size:u16) -> Vec<String> {
        self.matched_indices
            .iter()
            .enumerate()
            .map(|(i, &index)| {
                format!(
                    "{} {}: {}",
                    if i == self.active_line_number { "*" } else { " " },
                    index,
                    self.items[index].get_line()
                )
            })
            .collect()
    }

    pub fn get_active_item(&self) -> &I {
        &self.items[self.matched_indices[self.active_line_number]]
    }
}
