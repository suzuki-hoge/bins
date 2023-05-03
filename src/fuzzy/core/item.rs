use crate::fuzzy::core::guide::Label;
use crate::fuzzy::core::tab::Tab;
use tui::widgets::ListItem;

pub trait Item {
    fn get_line(&self) -> String;

    fn get_preview(&self) -> Vec<String>;

    fn custom_preview_style<S: Into<String>>(&self, s: S) -> ListItem {
        ListItem::new(s.into())
    }

    fn tab_filter(&self, _tab: &Tab) -> bool {
        true
    }

    fn can_activate_guide_label(&self, _label: &Label) -> bool {
        true
    }
}
