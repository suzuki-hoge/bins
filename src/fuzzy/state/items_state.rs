use crate::fuzzy::item::Item;

#[derive(Debug)]
pub struct ItemsState<I: Item> {
    items: Vec<I>,
    filter: String,
    active_line_number: usize,
}

impl<I: Item> ItemsState<I> {
    pub fn new(items: Vec<I>) -> Self {
        Self { items, filter: String::new(), active_line_number: 0 }
    }

    pub fn up(&mut self) {
        self.active_line_number -= 1;
    }

    pub fn down(&mut self) {
        self.active_line_number += 1;
    }

    pub fn update_filer(&mut self, input: &String) {
        self.filter = input.to_string();
    }

    pub fn get_matched_lines(&self) -> Vec<String> {
        self.items
            .iter()
            .filter(|item| item.get_line().contains(&self.filter))
            .enumerate()
            .map(|(n, item)| {
                format!("{} {}: {}", if n == self.active_line_number { "*" } else { " " }, n, item.get_line())
            })
            .collect()
    }

    pub fn get_active_item(&self) -> &I {
        &self.items[self.active_line_number]
    }
}
