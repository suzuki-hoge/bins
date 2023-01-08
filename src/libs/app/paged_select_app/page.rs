use itertools::Itertools;

#[derive(Debug)]
pub struct Page {
    per_page: usize,
    head_index_in_page: usize,
    last_index_in_page: usize,
    item_numbers_in_page: Vec<usize>,
    active_item_number: usize,
    active_item_index: usize,
}

impl Page {
    pub fn init(per_page: usize) -> Self {
        Self {
            per_page,
            head_index_in_page: 0,
            last_index_in_page: per_page,
            item_numbers_in_page: vec![],
            active_item_number: 0,
            active_item_index: 0,
        }
    }

    pub fn get_item_numbers_in_page(&self) -> &Vec<usize> {
        &self.item_numbers_in_page
    }

    pub fn get_active_item_number(&self) -> &usize {
        &self.active_item_number
    }

    pub fn page_top(&mut self, item_numbers: &[usize]) {
        self.active_item_index = 0;
        self.head_index_in_page = 0;
        self.last_index_in_page = self.head_index_in_page + self.per_page;
        self.refresh(item_numbers);
        self.refresh2(item_numbers);
    }

    pub fn up(&mut self, item_numbers: &[usize]) {
        if self.active_item_index == self.head_index_in_page {
            self.head_index_in_page -= 1;
            self.last_index_in_page = self.head_index_in_page + self.per_page;
        }
        self.active_item_index -= 1;
        self.refresh(item_numbers);
        self.refresh2(item_numbers);
    }

    pub fn down(&mut self, item_numbers: &[usize]) {
        if self.active_item_index == self.last_index_in_page - 1 {
            self.head_index_in_page += 1;
            self.last_index_in_page = self.head_index_in_page + self.per_page;
        }
        self.active_item_index += 1;
        self.refresh(item_numbers);
        self.refresh2(item_numbers);
    }

    fn refresh(&mut self, item_numbers: &[usize]) {
        self.active_item_number = item_numbers[self.active_item_index];
    }

    pub fn refresh2(&mut self, matched_item_numbers: &[usize]) {
        self.item_numbers_in_page = matched_item_numbers
            .iter()
            .enumerate()
            .filter(|&(item_index, _)| self.head_index_in_page <= item_index && item_index <= self.last_index_in_page)
            .map(|(_, matched_item)| *matched_item)
            .collect_vec();
    }
}
