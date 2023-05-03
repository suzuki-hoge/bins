use crate::fuzzy::core::guide::Label;
use crate::fuzzy::core::item::Item;

#[derive(Debug)]
pub struct GuideState {
    pub labels: Vec<(Label, bool)>,
}

impl GuideState {
    pub fn new(labels: Vec<Label>) -> Self {
        Self { labels: labels.into_iter().map(|label| (label, false)).collect() }
    }

    pub fn toggle<I: Item>(&mut self, item: &I, c: char) {
        if let Some(i) = self.labels.iter().position(|(label, _)| label.c == c) {
            if item.can_activate_guide_label(&self.labels[i].0) {
                self.labels[i].1 = !self.labels[i].1;
            }
        }
    }
}
