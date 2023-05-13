use itertools::Itertools;

use bins::fuzzy::core::item::Item;
use bins::io::command::get_command_out_lines;

pub fn gather_diffs(staged: bool) -> Vec<DiffItem> {
    let i = if staged { 0 } else { 1 };

    get_command_out_lines("git status -s")
        .unwrap()
        .iter()
        .map(|line| (line.chars().nth(i).unwrap(), line))
        .filter(|&(c, _)| c != ' ')
        .map(|(c, line)| DiffItem::new(c, line, staged))
        .collect()
}

#[derive(Clone, Debug)]
pub struct DiffItem {
    mark: char,
    file_path: String,
    diff: Vec<String>,
    staged: bool,
}

impl DiffItem {
    pub fn new<S: Into<String>>(mark: char, line: S, staged: bool) -> Self {
        let file_path = line.into().trim().replace("  ", " ").split(' ').collect_vec()[1].trim().to_string();
        let diff = if staged {
            get_command_out_lines(format!("git diff --no-color --staged {file_path}")).unwrap()
        } else {
            get_command_out_lines(format!("git diff --no-color {file_path}")).unwrap()
        };
        Self { mark, file_path, diff, staged }
    }
}

impl Item for DiffItem {
    fn get_line(&self) -> String {
        match self.staged {
            true => format!("{}  {}", self.mark, self.file_path),
            false => format!(" {} {}", self.mark, self.file_path),
        }
    }

    fn get_preview(&self) -> Vec<String> {
        self.diff.clone()
    }
}

#[cfg(test)]
mod tests {
    use crate::item::DiffItem;

    #[test]
    fn test() {
        let act = DiffItem::new('A', "A  Cargo.toml", true);
        assert_eq!(&act.file_path, "Cargo.toml");

        let act = DiffItem::new('A', " A Cargo.toml", false);
        assert_eq!(&act.file_path, "Cargo.toml");

        let act = DiffItem::new('A', "AM Cargo.toml", true);
        assert_eq!(&act.file_path, "Cargo.toml");

        let act = DiffItem::new('M', "AM Cargo.toml", false);
        assert_eq!(&act.file_path, "Cargo.toml");
    }
}
