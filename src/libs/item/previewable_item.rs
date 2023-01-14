use std::fmt::Debug;

pub trait PreviewableItem: Sized + Send + Sync + Eq + PartialEq + Clone + Debug {
    fn get_origin(&self) -> String;
    fn get_preview(&self) -> Vec<String>;
}

impl PreviewableItem for String {
    fn get_origin(&self) -> String {
        self.to_string()
    }

    fn get_preview(&self) -> Vec<String> {
        vec!["no preview".to_string()]
    }
}
