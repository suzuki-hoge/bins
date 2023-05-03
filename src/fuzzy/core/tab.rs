pub struct TabNames {
    pub names: Vec<String>,
}

impl TabNames {
    pub fn new(names: Vec<String>) -> Self {
        Self { names }
    }

    pub fn empty() -> Self {
        Self { names: vec![] }
    }
}

#[derive(Debug)]
pub struct Tab {
    pub current: usize,
    total: usize,
}

impl Tab {
    pub fn new(tab_names: &TabNames) -> Self {
        Self { current: 0, total: tab_names.names.len() }
    }

    pub fn next(&mut self) {
        if self.current == self.total - 1 {
            self.current = 0;
        } else {
            self.current += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.current == 0 {
            self.current = self.total - 1;
        } else {
            self.current -= 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fuzzy::core::tab::{Tab, TabNames};

    #[test]
    fn test() {
        let tab_names = TabNames::new(vec![String::from("Tab1"), String::from("Tab2"), String::from("Tab3")]);
        let mut tab = Tab::new(&tab_names);

        assert_eq!(tab.current, 0);
        assert_eq!(tab.total, 3);

        tab.next();
        assert_eq!(tab.current, 1);

        tab.next();
        assert_eq!(tab.current, 2);

        tab.next();
        assert_eq!(tab.current, 0);

        tab.prev();
        assert_eq!(tab.current, 2);

        tab.prev();
        assert_eq!(tab.current, 1);

        tab.prev();
        assert_eq!(tab.current, 0);
    }
}
