use bins::libs::git::config::GitConfig;
use bins::libs::item::display_item::DisplayItem;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct UrlItems<'a> {
    git_config: &'a GitConfig,
    items: Vec<UrlItem<'a>>,
}

impl<'a> UrlItems<'a> {
    pub fn create(git_config: &'a GitConfig) -> Self {
        UrlItems { git_config, items: vec![] }
    }

    pub fn add(&mut self, label: impl Into<String>, path: impl Into<String>) -> &Self {
        self.items.push(UrlItem {
            owner: &self.git_config.owner,
            repo: &self.git_config.repo,
            label: label.into(),
            path: path.into(),
        });
        self
    }

    pub fn get_items(&self) -> Vec<UrlItem> {
        self.items.clone()
    }

    pub fn get_raw(&self) -> String {
        if self.items.len() == 1 {
            self.items[0].get_raw()
        } else {
            unreachable!()
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct UrlItem<'a> {
    owner: &'a str,
    repo: &'a str,
    label: String,
    path: String,
}

impl<'a> UrlItem<'a> {
    pub fn get_preview(&self) -> String {
        format!("/{}", self.path)
    }

    pub fn get_raw(&self) -> String {
        format!("https://github.com/{}/{}/{}", self.owner, self.repo, self.path)
    }
}

impl<'a> DisplayItem for UrlItem<'a> {
    fn get_pane1(&self) -> String {
        self.label.clone()
    }

    fn get_pane2(&self) -> Vec<String> {
        vec![self.get_preview()]
    }

    fn is_editable(&self) -> bool {
        false
    }
}
