use crate::fuzzy::core::guide::Label;
use crate::fuzzy::core::tab::Tab;
use std::fmt::Debug;

pub trait Item: Sized + Send + Clone + Debug {
    fn get_line(&self) -> String;

    fn get_preview(&self) -> Vec<String> {
        vec![]
    }

    fn tab_filter(&self, _: &Tab) -> bool {
        true
    }

    fn can_activate_guide_label(&self, _: &Label) -> bool {
        true
    }
}

impl Item for String {
    fn get_line(&self) -> String {
        self.to_string()
    }

    fn get_preview(&self) -> Vec<String> {
        vec![]
    }
}
