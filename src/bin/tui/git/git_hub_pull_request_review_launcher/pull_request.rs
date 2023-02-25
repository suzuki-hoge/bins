use pad::PadStr;

use bins::libs::git::config::GitConfig;
use bins::libs::item::display_item::DisplayItem;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct PullRequest {
    number: u64,
    max_number: u64,
    author: String,
    max_author_length: usize,
    branch: String,
    max_branch_length: usize,
    title: String,
    requesting_reviewers: Vec<String>,
    commented_reviewers: Vec<String>,
}

impl PullRequest {
    pub fn new(
        (number, max_number): (u64, u64),
        (author, max_author_length): (String, usize),
        (branch, max_branch_length): (String, usize),
        title: String,
        requesting_reviewers: Vec<String>,
        commented_reviewers: Vec<String>,
    ) -> Self {
        Self {
            number,
            max_number,
            author,
            max_author_length,
            branch,
            max_branch_length,
            title,
            requesting_reviewers,
            commented_reviewers,
        }
    }

    pub fn get_number(&self) -> u64 {
        self.number
    }

    pub fn get_url(&self, git_config: &GitConfig) -> String {
        format!("https://github.com/{}/{}/pull/{}", git_config.owner, git_config.repo, self.number)
    }

    pub fn is_own(&self, username: &String) -> bool {
        &self.author == username
    }

    pub fn is_not_reviewed(&self, username: &String) -> bool {
        self.requesting_reviewers.contains(username)
    }

    pub fn is_reviewed(&self, username: &String) -> bool {
        !self.is_own(username) && self.commented_reviewers.contains(username)
    }
}

impl DisplayItem for PullRequest {
    fn get_pane1(&self) -> String {
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

    fn get_pane2(&self) -> Vec<String> {
        vec![]
    }

    fn is_editable(&self) -> bool {
        false
    }
}

#[cfg(test)]
mod tests {
    use trim_margin::MarginTrimmable;

    use bins::libs::item::display_item::DisplayItem;

    use crate::pull_request::PullRequest;

    #[test]
    fn new() {
        let sut1 = PullRequest::new(
            (1, 42),
            ("suzuki-hoge".to_string(), 11),
            ("feature/auth".to_string(), 20),
            "bla bla bla".to_string(),
            vec![],
            vec![],
        );

        let sut2 = PullRequest::new(
            (42, 42),
            ("john".to_string(), 11),
            ("feature/csv-download".to_string(), 20),
            "lorem ipsum".to_string(),
            vec![],
            vec![],
        );

        let act = vec![sut1.get_pane1(), sut2.get_pane1()].join("\n");
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
