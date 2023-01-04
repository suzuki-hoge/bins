use itertools::Itertools;

#[derive(Debug)]
pub struct Page {
    page: usize,
    per_page: usize,
    head_index_in_page: usize,
    last_index_in_page: usize,
    item_numbers_in_page: Vec<usize>,
}

impl Page {
    pub fn init() -> Self {
        Self { page: 0, per_page: 20, head_index_in_page: 0, last_index_in_page: 0, item_numbers_in_page: vec![] }
    }

    pub fn get_item_numbers_in_page(&self) -> &Vec<usize> {
        &self.item_numbers_in_page
    }

    pub fn get_head_index_in_page(&self) -> &usize {
        &self.head_index_in_page
    }

    pub fn is_out_of_page(&self, item_index: &usize) -> bool {
        item_index < &self.head_index_in_page || &self.last_index_in_page < item_index
    }

    pub fn turn_top(&mut self) {
        self.page = 0;
    }

    pub fn turn_prev(&mut self, matched_item_numbers: &[usize]) {
        self.page -= 1;
        self.refresh(matched_item_numbers);
    }

    pub fn turn_next(&mut self, matched_item_numbers: &[usize]) {
        self.page += 1;
        self.refresh(matched_item_numbers);
    }

    pub fn refresh(&mut self, matched_item_numbers: &[usize]) {
        self.last_index_in_page = self.per_page * (self.page + 1) - 1;
        self.head_index_in_page = self.last_index_in_page + 1 - self.per_page;
        self.item_numbers_in_page = matched_item_numbers
            .iter()
            .enumerate()
            .filter(|&(item_index, _)| self.head_index_in_page <= item_index && item_index <= self.last_index_in_page)
            .map(|(_, matched_item)| *matched_item)
            .collect_vec();
    }
}
