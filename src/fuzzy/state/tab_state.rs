use crate::fuzzy::core::tab::Tab;

#[derive(Debug)]
pub struct TabState {
    pub tab: Tab,
}

impl TabState {
    pub fn new(tab: Tab) -> Self {
        Self { tab }
    }
}
