use crate::item::UrlType::{All, BaseBranch, CurrentBranch};
use bins::fuzzy::core::item::Item;
use bins::fuzzy::core::tab::Tab;
use bins::git::branch::GitBranch;
use bins::git::config::{get_current_git_config, GitConfig};
use bins::io::command::get_command_out_lines;

pub fn gather_urls(git_branch: &GitBranch) -> Vec<UrlItem> {
    if let Some(git_config) = get_current_git_config() {
        let mut all_def = vec![
            UrlItem::from_def("pulls", "/pulls", &git_config, All),
            UrlItem::from_def("my pulls", "/pulls/@me", &git_config, All),
            UrlItem::from_def(
                "review pulls",
                "/pulls?q=is:open+is:pr+-reviewed-by:@me+reviewed-by:@me",
                &git_config,
                All,
            ),
            UrlItem::from_def("issues", "/issues", &git_config, All),
            UrlItem::from_def("my issues", "/issues/assigned/@me", &git_config, All),
            UrlItem::from_def("actions", "/actions", &git_config, All),
            UrlItem::from_def("releases", "/releases", &git_config, All),
            UrlItem::from_def("tags", "/tags", &git_config, All),
            UrlItem::from_def("wiki", "/wiki", &git_config, All),
        ];
        let mut current_def = vec![
            UrlItem::from_def("files", format!("/tree/{}", git_branch.current), &git_config, CurrentBranch),
            UrlItem::from_def("commits", format!("/commits/{}", git_branch.current), &git_config, CurrentBranch),
            UrlItem::from_def("find", format!("/find/{}", git_branch.current), &git_config, CurrentBranch),
        ];
        let mut base_def = if let Some(base) = git_branch.base.as_ref() {
            vec![
                UrlItem::from_def("files", format!("/tree/{base}"), &git_config, BaseBranch),
                UrlItem::from_def("commits", format!("/commits/{base}"), &git_config, BaseBranch),
                UrlItem::from_def("find", format!("/find/{base}"), &git_config, BaseBranch),
            ]
        } else {
            vec![]
        };
        let mut current_file_path = get_command_out_lines("git ls-files")
            .unwrap()
            .into_iter()
            .map(|path| UrlItem::from_file_path(&git_branch.current, path, &git_config, CurrentBranch))
            .collect();
        let mut base_file_path = if let Some(base) = git_branch.base.as_ref() {
            get_command_out_lines("git ls-files")
                .unwrap()
                .into_iter()
                .map(|path| UrlItem::from_file_path(base, path, &git_config, BaseBranch))
                .collect()
        } else {
            vec![]
        };

        let mut items = vec![];
        items.append(&mut all_def);
        items.append(&mut current_def);
        items.append(&mut base_def);
        items.append(&mut current_file_path);
        items.append(&mut base_file_path);
        items
    } else {
        vec![]
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum UrlType {
    All,
    CurrentBranch,
    BaseBranch,
}

#[derive(Clone, Debug)]
pub struct UrlItem {
    label: String,
    url_type: UrlType,
    pub url: String,
}

impl UrlItem {
    fn from_def<S: Into<String>>(label: &str, path: S, git_config: &GitConfig, url_type: UrlType) -> Self {
        let label = label.to_string();
        let path = path.into();
        let url = format!("https://github.com/{}/{}{}", git_config.owner, git_config.repo, path);
        Self { label, url_type, url }
    }

    fn from_file_path(branch: &str, file_path: String, git_config: &GitConfig, url_type: UrlType) -> Self {
        let label = file_path.to_string();
        let path = format!("/blob/{branch}/{file_path}");
        let url = format!("https://github.com/{}/{}{}", git_config.owner, git_config.repo, path);
        Self { label, url_type, url }
    }
}

impl Item for UrlItem {
    fn get_line(&self) -> String {
        self.label.to_string()
    }

    fn tab_filter(&self, tab: &Tab) -> bool {
        match tab.current {
            0 => self.url_type == All,
            1 => self.url_type == CurrentBranch,
            2 => self.url_type == BaseBranch,
            _ => panic!(),
        }
    }
}
