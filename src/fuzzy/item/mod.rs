use tui::widgets::ListItem;

pub trait Item {
    fn get_line(&self) -> String;

    fn get_preview(&self) -> Vec<String>;

    fn custom_preview_style<S: Into<String>>(&self, s: S) -> ListItem {
        ListItem::new(s.into())
    }

    fn get_tab_names() -> Vec<String>;

    fn shift_tab(&mut self);
}
