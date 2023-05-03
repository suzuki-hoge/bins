use crate::fuzzy::core::guide::Label;
use crate::fuzzy::core::item::Item;

#[derive(Debug)]
pub struct GuideState {
    pub labels: Vec<(Label, bool)>,
}

impl GuideState {
    pub fn new(labels: Vec<Label>) -> Self {
        Self { labels: labels.into_iter().map(|label| (label, false)).collect() }
    }

    pub fn toggle<I: Item>(&mut self, item: &I, c: char) {
        if let Some(i) = self.labels.iter().position(|(label, _)| label.c == c) {
            if item.can_activate_guide_label(&self.labels[i].0) {
                self.labels[i].1 = !self.labels[i].1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::fuzzy::core::guide::{Guide, Label};
    use crate::fuzzy::core::item::Item;
    use crate::fuzzy::state::guide_state::GuideState;

    struct TestItem {
        value: usize,
    }

    impl Item for TestItem {
        fn get_line(&self) -> String {
            String::new()
        }

        fn get_preview(&self) -> Vec<String> {
            vec![]
        }

        fn can_activate_guide_label(&self, label: &Label) -> bool {
            match (self.value, label.value) {
                (1, _) => true,
                (2, "Edit") => true,
                (_, _) => false,
            }
        }
    }

    #[test]
    fn test_all_enabling() {
        let item = TestItem { value: 1 };

        let mut sut = GuideState::new(Guide::new(vec!["Edit", "Run"]).labels);

        assert!(!sut.labels[0].1);
        assert!(!sut.labels[1].1);

        sut.toggle(&item, 'E');

        assert!(sut.labels[0].1);
        assert!(!sut.labels[1].1);

        sut.toggle(&item, 'R');

        assert!(sut.labels[0].1);
        assert!(sut.labels[1].1);

        sut.toggle(&item, 'E');

        assert!(!sut.labels[0].1);
        assert!(sut.labels[1].1);

        sut.toggle(&item, 'R');

        assert!(!sut.labels[0].1);
        assert!(!sut.labels[1].1);
    }

    #[test]
    fn test_partial_enabling() {
        let item = TestItem { value: 2 };

        let mut sut = GuideState::new(Guide::new(vec!["Edit", "Run"]).labels);

        assert!(!sut.labels[0].1);
        assert!(!sut.labels[1].1);

        sut.toggle(&item, 'E');

        assert!(sut.labels[0].1);
        assert!(!sut.labels[1].1);

        sut.toggle(&item, 'R');

        assert!(sut.labels[0].1);
        assert!(!sut.labels[1].1);

        sut.toggle(&item, 'E');

        assert!(!sut.labels[0].1);
        assert!(!sut.labels[1].1);

        sut.toggle(&item, 'R');

        assert!(!sut.labels[0].1);
        assert!(!sut.labels[1].1);
    }
}
