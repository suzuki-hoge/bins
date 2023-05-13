use tui::widgets::ListItem;

pub trait CustomPreviewStyle {
    fn is_default(&self) -> bool {
        false
    }

    fn to_list_item(&self, s: String) -> ListItem;
}

pub struct DefaultPreviewStyle {}

impl CustomPreviewStyle for DefaultPreviewStyle {
    fn is_default(&self) -> bool {
        true
    }
    fn to_list_item(&self, s: String) -> ListItem {
        ListItem::new(s)
    }
}
