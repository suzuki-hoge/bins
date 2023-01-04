use itertools::Itertools;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Debug)]
pub struct PagedSelectApp<Item: Clone + Debug, MatchedItem: Clone + Debug> {
    all_items: HashMap<usize, Item>,
    matched_items: HashMap<usize, MatchedItem>,
    matched_item_numbers: Vec<usize>,
    item_numbers_in_page: Vec<usize>,
    active_item_number: usize,
    page: usize,
    per_page: usize,
    head_index_in_page: usize,
    last_index_in_page: usize,
    active_item_index: usize,
}

impl<Item: Clone + Debug, MatchedItem: Clone + Debug> PagedSelectApp<Item, MatchedItem> {
    pub fn init(items: Vec<Item>) -> Self {
        let mut all_items: HashMap<usize, Item> = HashMap::new();
        items.into_iter().enumerate().for_each(|(i, item)| {
            all_items.insert(i, item);
        });
        Self {
            all_items,
            matched_items: HashMap::new(),
            matched_item_numbers: vec![],
            item_numbers_in_page: vec![],
            active_item_number: 0,
            page: 0,
            per_page: 1,
            head_index_in_page: 0,
            last_index_in_page: 0,
            active_item_index: 0,
        }
    }

    pub fn set_per_page(&mut self, per_page: u16) {
        self.per_page = per_page as usize;
        self.re_page();
    }

    pub fn get_matched_items_in_page(&self) -> Vec<(usize, MatchedItem)> {
        self.item_numbers_in_page
            .iter()
            .map(|item_number| (*item_number, self.matched_items.get(item_number).unwrap().clone()))
            .collect_vec()
    }

    pub fn is_active_item_number(&self, item_number: usize) -> bool {
        self.active_item_number == item_number
    }

    pub fn get_item(&self) -> Item {
        self.all_items[&self.active_item_number].clone()
    }

    pub fn pop_item(&mut self) -> Item {
        let last_one = self.matched_item_numbers[self.matched_item_numbers.len() - 1] == self.active_item_number;
        self.matched_item_numbers = self
            .matched_item_numbers
            .iter()
            .filter(|&item_number| item_number != &self.active_item_number)
            .map(|item_number| *item_number)
            .collect();
        self.re_page();
        self.matched_items.remove(&self.active_item_number);
        let item = self.all_items.remove(&self.active_item_number).unwrap();
        if last_one {
            self.active_item_index -= 1;
            self.active_item_number = self.matched_item_numbers[self.active_item_index];
        } else {
            self.active_item_number = self.matched_item_numbers[self.active_item_index];
        }
        item
    }

    pub fn up(&mut self) {
        if self.matched_item_numbers[0] != self.active_item_number {
            self.active_item_index -= 1;
            self.active_item_number = self.matched_item_numbers[self.active_item_index];
        }
        if self.active_item_index < self.head_index_in_page {
            self.page -= 1;
            self.re_page();
        }
    }

    pub fn down(&mut self) {
        if self.matched_item_numbers[self.matched_item_numbers.len() - 1] != self.active_item_number {
            self.active_item_index += 1;
            self.active_item_number = self.matched_item_numbers[self.active_item_index];
        }
        if self.last_index_in_page < self.active_item_index {
            self.page += 1;
            self.re_page();
        }
    }

    pub fn re_match<Matcher: Fn(Item) -> Option<MatchedItem>>(&mut self, matcher: Matcher) {
        self.matched_item_numbers = vec![];
        self.matched_items = HashMap::new();

        for item_number in self.all_items.keys() {
            let matched_item = matcher(self.all_items.get(item_number).unwrap().clone());

            if matched_item.is_some() {
                self.matched_item_numbers.push(*item_number);
                self.matched_items.insert(*item_number, matched_item.unwrap());
            }
        }
        self.matched_item_numbers.sort();
    }

    pub fn re_render(&mut self) {
        self.re_page();
        self.active_item_index = 0;
        self.active_item_number = self.matched_item_numbers[self.active_item_index];
    }

    pub fn re_page(&mut self) {
        self.last_index_in_page = self.per_page * (self.page + 1) - 1;
        self.head_index_in_page = self.last_index_in_page + 1 - self.per_page;
        self.item_numbers_in_page = self
            .matched_item_numbers
            .iter()
            .enumerate()
            .filter(|&(item_index, _)| self.head_index_in_page <= item_index && item_index <= self.last_index_in_page)
            .map(|(_, matched_item)| *matched_item)
            .collect_vec();
    }
}

#[cfg(test)]
mod tests {
    use crate::libs::app::paged_select_app::PagedSelectApp;
    use itertools::Itertools;

    #[test]
    fn edit() {
        let mut app = PagedSelectApp::init(('a'..'k').collect_vec());
        app.set_per_page(2);
        app.re_match(|c| if vec!['a', 'c', 'e', 'g', 'i'].contains(&c) { Some(c) } else { None });
        app.re_page();

        println!("{:?}", &app.all_items);
        println!("{:?}", &app.matched_items);
        println!("{:?}", &app.matched_item_numbers);
        println!("{:?}", &app.item_numbers_in_page);
        println!("{:?}", &app.get_matched_items_in_page());

        println!("{:?} ( active number )", &app.active_item_number);
        println!("{:?} ( page )", &app.page);

        app.down();

        println!("{:?} ( active number )", &app.active_item_number);
        println!("{:?} ( page )", &app.page);

        app.down();

        println!("{:?} ( active number )", &app.active_item_number);
        println!("{:?} ( page )", &app.page);

        app.down();

        println!("{:?} ( active number )", &app.active_item_number);
        println!("{:?} ( page )", &app.page);

        app.down();

        println!("{:?} ( active number )", &app.active_item_number);
        println!("{:?} ( page )", &app.page);

        app.down();

        println!("{:?} ( active number )", &app.active_item_number);
        println!("{:?} ( page )", &app.page);
        println!("{:?}", &app.get_matched_items_in_page());
    }
}
