use std::env::current_dir;
use std::path::{Path, PathBuf};

use crate::libs::io::reader::read_deserializable;
use crate::libs::io::writer::write_serializable;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Eq, PartialEq, Debug)]
pub struct BranchMemo {
    pub base: String,
    pub current: String,
}

pub fn get_branch_memos() -> anyhow::Result<Vec<BranchMemo>> {
    let bins_dir = PathBuf::from(std::env::var("HOME")?);
    let work_dir = current_dir()?;

    _get_branch_memos(&bins_dir, &work_dir)
}

fn _get_branch_memos(bins_dir: &Path, work_dir: &Path) -> anyhow::Result<Vec<BranchMemo>> {
    let work_dir_dot = work_dir.display().to_string().replace('/', ".");
    let yaml_path = bins_dir.join(".bins-branch").join(format!("{work_dir_dot}.yaml"));

    match read_deserializable(&yaml_path) {
        Ok(memos) => Ok(memos),
        Err(_) => Ok(vec![]),
    }
}

pub fn add_branch_memo(base: impl Into<String>, current: impl Into<String>) -> anyhow::Result<()> {
    let bins_dir = PathBuf::from(std::env::var("HOME")?);
    let work_dir = current_dir()?;

    _add_branch_memo(&bins_dir, &work_dir, &base.into(), &current.into())
}

pub fn _add_branch_memo(bins_dir: &Path, work_dir: &Path, base: &str, current: &str) -> anyhow::Result<()> {
    let base = base.into();
    let current = current.into();

    let work_dir_dot = work_dir.display().to_string().replace('/', ".");
    let yaml_path = bins_dir.join(".bins-branch").join(format!("{work_dir_dot}.yaml"));

    let mut memos = match read_deserializable(&yaml_path) {
        Ok(memos) => memos,
        Err(_) => vec![],
    };
    memos.push(BranchMemo { base, current });

    write_serializable(&yaml_path, &memos)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    use crate::libs::git::branch_memo::{BranchMemo, _add_branch_memo, _get_branch_memos};
    use trim_margin::MarginTrimmable;

    fn setup(bins_dir: &PathBuf) {
        let raw = "
            |- { base: develop, current: feat }
            |- { base: feat, current: sub-feat }
        "
        .trim()
        .trim_margin()
        .unwrap();

        let _ = fs::create_dir_all(bins_dir);
        let _ = fs::create_dir_all(bins_dir.join(".bins-branch"));
        let _ = File::create(bins_dir.join(".bins-branch").join(".path.front.yaml")).unwrap().write_all(raw.as_bytes());
    }

    fn cleanup(bins_dir: &PathBuf) {
        let _ = fs::remove_dir_all(bins_dir);
    }

    #[test]
    fn found() {
        let bins_dir = PathBuf::from("target/git/test-pj/memo-found");

        setup(&bins_dir);

        let work_dir = PathBuf::from("/path/front");

        let act = _get_branch_memos(&bins_dir, &work_dir);
        assert_eq!(
            act.unwrap(),
            vec![
                BranchMemo { base: "develop".to_string(), current: "feat".to_string() },
                BranchMemo { base: "feat".to_string(), current: "sub-feat".to_string() }
            ]
        );

        let _ = _add_branch_memo(&bins_dir, &work_dir, "feat", "sub-feat2");

        let act = _get_branch_memos(&bins_dir, &work_dir);
        assert_eq!(
            act.unwrap(),
            vec![
                BranchMemo { base: "develop".to_string(), current: "feat".to_string() },
                BranchMemo { base: "feat".to_string(), current: "sub-feat".to_string() },
                BranchMemo { base: "feat".to_string(), current: "sub-feat2".to_string() }
            ]
        );

        cleanup(&bins_dir);
    }

    #[test]
    fn notfound() {
        let bins_dir = PathBuf::from("target/git/test-pj/memo-notfound");

        let work_dir = PathBuf::from("/path/front");

        let act = _get_branch_memos(&bins_dir, &work_dir);
        assert_eq!(act.unwrap(), vec![]);

        let _x = _add_branch_memo(&bins_dir, &work_dir, "feat", "sub-feat2");

        let act = _get_branch_memos(&bins_dir, &work_dir);
        assert_eq!(act.unwrap(), vec![BranchMemo { base: "feat".to_string(), current: "sub-feat2".to_string() }]);

        cleanup(&bins_dir);
    }
}
