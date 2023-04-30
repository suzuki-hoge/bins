pub trait Item {
    fn get_line(&self) -> String;

    fn get_preview(&self) -> String;

    fn get_tab_names() -> Vec<String>;

    fn shift_tab(&mut self);
}
