use pad::PadStr;

use bins::fuzzy::core::item::Item;
use bins::fuzzy::core::tab::Tab;
use bins::git::config::GitConfig;
use bins::git::username::get_git_username;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PullRequestItem {
    number: u64,
    max_number: u64,
    author: String,
    max_author_length: usize,
    branch: String,
    max_branch_length: usize,
    title: String,
    requesting_reviewers: Vec<String>,
    commented_reviewers: Vec<String>,
    me: String,
    url: String,
}

impl PullRequestItem {
    pub fn new<S: Into<String>>(
        git_config: &GitConfig,
        (number, max_number): (u64, u64),
        (author, max_author_length): (S, usize),
        (branch, max_branch_length): (S, usize),
        title: S,
        requesting_reviewers: Vec<String>,
        commented_reviewers: Vec<String>,
    ) -> Self {
        let url = format!("https://github.com/{}/{}/pull/{}", git_config.owner, git_config.repo, number);
        let me = get_git_username();
        Self {
            number,
            max_number,
            author: author.into(),
            max_author_length,
            branch: branch.into(),
            max_branch_length,
            title: title.into(),
            requesting_reviewers,
            commented_reviewers,
            me,
            url,
        }
    }

    pub fn get_number(&self) -> u64 {
        self.number
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    fn is_own(&self) -> bool {
        self.author == self.me
    }

    fn is_not_reviewed(&self) -> bool {
        self.requesting_reviewers.contains(&self.me)
    }

    fn is_reviewed(&self) -> bool {
        !self.is_own() && self.commented_reviewers.contains(&self.me)
    }
}

impl Item for PullRequestItem {
    fn get_line(&self) -> String {
        let number_digit = ((self.number as f64).log10() + 1.0) as usize;
        let max_number_digit = ((self.max_number as f64).log10() + 1.0) as usize;
        format!(
            "{}#{}    {}    {}    {}",
            " ".repeat(max_number_digit - number_digit),
            self.number,
            self.author.pad_to_width(self.max_author_length),
            self.branch.pad_to_width(self.max_branch_length),
            self.title
        )
    }

    fn tab_filter(&self, tab: &Tab) -> bool {
        match tab.current {
            0 => true,
            1 => self.is_own(),
            2 => self.is_not_reviewed(),
            3 => self.is_reviewed(),
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use trim_margin::MarginTrimmable;

    use bins::fuzzy::core::item::Item;
    use bins::git::config::get_git_config;
    use bins::io::command::get_command_out_line;

    use crate::item::pull_request_item::PullRequestItem;

    #[test]
    fn new() {
        let bins = get_command_out_line("git rev-parse --show-toplevel").unwrap();
        let git_config = get_git_config(bins).unwrap();

        let sut1 = PullRequestItem::new(
            &git_config,
            (1, 42),
            ("suzuki-hoge", 11),
            ("feature/auth", 20),
            "bla bla bla",
            vec![],
            vec![],
        );

        let sut2 = PullRequestItem::new(
            &git_config,
            (42, 42),
            ("john", 11),
            ("feature/csv-download", 20),
            "lorem ipsum",
            vec![],
            vec![],
        );

        let act = vec![sut1.get_line(), sut2.get_line()].join("\n");
        let exp = "
            | #1    suzuki-hoge    feature/auth            bla bla bla
            |#42    john           feature/csv-download    lorem ipsum
        "
        .trim()
        .trim_margin()
        .unwrap();
        assert_eq!(act, exp);
    }
}
