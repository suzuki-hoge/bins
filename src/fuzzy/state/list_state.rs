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
        if self.active_line_number == 0 {
            self.active_line_number = self.matched_indices.len() - 1;
        } else {
            self.active_line_number -= 1;
        }
    }

    pub fn down(&mut self) {
        if self.active_line_number == self.matched_indices.len() - 1 {
            self.active_line_number = 0;
        } else {
            self.active_line_number += 1;
        }
    }

    pub fn get_active_item(&self) -> Option<&I> {
        if self.active_line_number < self.matched_indices.len() {
            Some(&self.items[self.matched_indices[self.active_line_number]])
        } else {
            None
        }
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

    pub fn get_matched_line_parts(&self, page_size: u16) -> Vec<(bool, Vec<(String, bool)>)> {
        let range = range(self.active_line_number, page_size);

        self.matched_indices
            .iter()
            .enumerate()
            .filter(|(line_number, _)| range.0 <= *line_number && *line_number <= range.1)
            .map(|(line_number, &index)| {
                (line_number == self.active_line_number, self.matcher.get_matched_parts(&self.items[index].get_line()))
            })
            .collect()
    }
}

fn range(line_number: usize, page_size: u16) -> (usize, usize) {
    let page_size = page_size as usize;
    let s = line_number / page_size * page_size;
    (s, s + page_size - 1)
}

#[cfg(test)]
mod tests {
    use crate::fuzzy::state::list_state::range;

    #[test]
    fn range_test() {
        assert_eq!(range(0, 4), (0, 3));
        assert_eq!(range(1, 4), (0, 3));
        assert_eq!(range(2, 4), (0, 3));
        assert_eq!(range(3, 4), (0, 3));

        assert_eq!(range(4, 4), (4, 7));
        assert_eq!(range(5, 4), (4, 7));
        assert_eq!(range(6, 4), (4, 7));
        assert_eq!(range(7, 4), (4, 7));
    }
}
