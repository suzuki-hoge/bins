use crate::fuzzy::core::guide::{Guide, Label};
use crate::fuzzy::core::item::Item;

#[derive(Debug)]
pub struct GuideState {
    pub labels: Vec<(Label, bool)>,
}

impl GuideState {
    pub fn new(guide: Guide) -> Self {
        Self {
            labels: guide
                .labels
                .into_iter()
                .enumerate()
                .map(|(i, label)| (label, guide.actives.contains(&i)))
                .collect(),
        }
    }

    pub fn toggle<I: Item>(&mut self, item: &I, c: char) {
        if let Some(i) = self.labels.iter().position(|(label, _)| label.c == c) {
            if item.can_activate_guide_label(&self.labels[i].0) {
                self.labels[i].1 = !self.labels[i].1;
            }
        }
    }

    pub fn get_active_chars(&self) -> Vec<char> {
        self.labels.iter().filter(|(_, b)| *b).map(|(label, _)| label.c).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::fuzzy::core::guide::{Guide, Label};
    use crate::fuzzy::core::item::Item;
    use crate::fuzzy::state::guide_state::GuideState;

    #[derive(Clone, Debug)]
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
            match (self.value, label.c) {
                (1, _) => true,
                (2, 'E') => true,
                (_, _) => false,
            }
        }
    }

    #[test]
    fn test_all_enabling() {
        let item = TestItem { value: 1 };

        let mut sut = GuideState::new(Guide::new(vec!["Edit", "Run"], vec![]));

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

        let mut sut = GuideState::new(Guide::new(vec!["Edit", "Run"], vec![]));

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

    #[test]
    fn test_actives() {
        let sut = GuideState::new(Guide::new(vec!["Edit", "Run"], vec![1]));

        assert!(!sut.labels[0].1);
        assert!(sut.labels[1].1);
    }
}
