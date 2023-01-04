#[derive(Debug)]
pub struct Active {
    item_number: usize,
    item_index: usize,
}

impl Active {
    pub fn init() -> Self {
        Self { item_number: 0, item_index: 0 }
    }

    pub fn get_item_number(&self) -> &usize {
        &self.item_number
    }

    pub fn get_item_index(&self) -> &usize {
        &self.item_index
    }

    pub fn page_top(&mut self, head_index_in_page: &usize, item_numbers: &[usize]) {
        self.item_index = *head_index_in_page;
        self.item_number = item_numbers[self.item_index];
    }

    pub fn up(&mut self, item_numbers: &[usize]) {
        self.item_index -= 1;
        self.refresh(item_numbers);
    }

    pub fn down(&mut self, item_numbers: &[usize]) {
        self.item_index += 1;
        self.refresh(item_numbers);
    }

    pub fn refresh(&mut self, item_numbers: &[usize]) {
        self.item_number = item_numbers[self.item_index];
    }
}
