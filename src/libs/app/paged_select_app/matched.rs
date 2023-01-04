use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct Matched<MatchedItem> {
    items: HashMap<usize, MatchedItem>,
    item_numbers: Vec<usize>,
}

impl<MatchedItem> Matched<MatchedItem> {
    pub fn init() -> Self {
        Self { items: HashMap::new(), item_numbers: vec![] }
    }

    pub fn get_item<'a>(&'a self, target_item_number: &'a usize) -> (&'a usize, &'a MatchedItem) {
        (target_item_number, self.items.get(target_item_number).unwrap())
    }

    pub fn get_item_numbers(&self) -> &Vec<usize> {
        &self.item_numbers
    }

    pub fn is_empty(&self) -> bool {
        self.item_numbers.is_empty()
    }

    pub fn is_head_number(&self, item_number: &usize) -> bool {
        &self.item_numbers[0] == item_number
    }

    pub fn is_last_number(&self, item_number: &usize) -> bool {
        &self.item_numbers[self.item_numbers.len() - 1] == item_number
    }

    pub fn refresh<Item, Matcher>(&mut self, all_items: &HashMap<usize, Item>, matcher: Matcher)
    where
        Matcher: Fn(&Item) -> Option<MatchedItem>,
    {
        self.item_numbers = vec![];
        self.items = HashMap::new();

        for item_number in all_items.keys() {
            let matched_item = matcher(all_items.get(item_number).unwrap());

            matched_item.into_iter().for_each(|item| {
                self.item_numbers.push(*item_number);
                self.items.insert(*item_number, item);
            })
        }
        self.item_numbers.sort_unstable();
    }

    pub fn remove(&mut self, target_item_number: &usize) {
        self.item_numbers.retain(|item_number| item_number != target_item_number);
        self.items.remove(target_item_number);
    }
}
