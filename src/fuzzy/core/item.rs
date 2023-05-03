use crate::fuzzy::core::guide::Label;
use crate::fuzzy::core::tab::Tab;
use tui::widgets::ListItem;

pub trait Item {
    fn get_line(&self) -> String;

    fn get_preview(&self) -> Vec<String>;

    fn custom_preview_style(&self, s: String) -> ListItem {
        ListItem::new(s)
    }

    fn tab_filter(&self, _: &Tab) -> bool {
        true
    }

    fn can_activate_guide_label(&self, _: &Label) -> bool {
        true
    }
}
