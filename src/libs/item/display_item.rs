use std::fmt::Debug;

pub trait DisplayItem: Sized + Send + Sync + Eq + PartialEq + Clone + Debug {
    fn get_pane1(&self) -> String;

    fn get_pane2(&self) -> Vec<String>;

    fn is_editable(&self) -> bool;
}

impl DisplayItem for String {
    fn get_pane1(&self) -> String {
        self.to_string()
    }

    fn get_pane2(&self) -> Vec<String> {
        vec![]
    }

    fn is_editable(&self) -> bool {
        false
    }
}
