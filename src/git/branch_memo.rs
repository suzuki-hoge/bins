use std::collections::HashMap;
use std::env::current_dir;
use std::path::{Path, PathBuf};

use crate::io::file::{get_bins_dir, read_deserializable, write_serializable};
use serde::{Deserialize, Serialize};

type Yaml = HashMap<String, Vec<BranchMemo>>;

#[derive(Serialize, Deserialize, Eq, PartialEq, Clone, Debug)]
pub struct BranchMemo {
    pub base: String,
    pub current: String,
}

pub fn get_branch_memos() -> anyhow::Result<Vec<BranchMemo>> {
    _get_branch_memos(get_bins_dir(), current_dir()?.to_string_lossy())
}

fn _get_branch_memos<P: AsRef<Path>, S: Into<String>>(bins_dir: P, work_dir: S) -> anyhow::Result<Vec<BranchMemo>> {
    let yaml_path = bins_dir.as_ref().join("branch-memo");

    match read_deserializable::<PathBuf, Yaml>(yaml_path) {
        Ok(yaml) => Ok(yaml.get(&work_dir.into()).cloned().unwrap_or(vec![])),
        Err(_) => Ok(vec![]),
    }
}

pub fn add_branch_memo(base: String, current: String) -> anyhow::Result<()> {
    _add_branch_memo(get_bins_dir(), current_dir()?.to_string_lossy(), base, current)
}

fn _add_branch_memo<P: AsRef<Path>, S: Into<String>>(
    bins_dir: P,
    work_dir: S,
    base: String,
    current: String,
) -> anyhow::Result<()> {
    let yaml_path = bins_dir.as_ref().join("branch-memo");

    match read_deserializable::<&PathBuf, Yaml>(&yaml_path) {
        Ok(mut yaml) => {
            let e = yaml.entry(work_dir.into()).or_insert(vec![]);
            e.push(BranchMemo { base, current });
            write_serializable(&yaml_path, &yaml)
        }
        Err(_) => {
            let mut yaml = HashMap::new();
            yaml.insert(work_dir.into(), vec![BranchMemo { base, current }]);
            write_serializable(&yaml_path, &yaml)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::fs::File;
    use std::io::Write;
    use std::path::{Path, PathBuf};

    use crate::git::branch_memo::{BranchMemo, _add_branch_memo, _get_branch_memos};
    use trim_margin::MarginTrimmable;

    fn setup<P: AsRef<Path>>(bins_dir: P) {
        let raw = "
            |/proj/1:
            |  - base: develop
            |    current: feat
            |  - base: feat
            |    current: sub-feat
        "
        .trim()
        .trim_margin()
        .unwrap();

        let bins_dir = bins_dir.as_ref();
        let _ = fs::create_dir_all(bins_dir);
        let _ = File::create(bins_dir.join("branch-memo")).unwrap().write_all(raw.as_bytes());
    }

    fn cleanup<P: AsRef<Path>>(dir: P) {
        let _ = fs::remove_dir_all(dir);
    }

    fn s(s: &str) -> String {
        s.to_string()
    }

    #[test]
    fn found() {
        let bins_dir = PathBuf::from("target/git/test-project/memo-found");

        setup(&bins_dir);

        let work_dir = "/proj/1";

        let act = _get_branch_memos(&bins_dir, work_dir);
        assert_eq!(
            act.unwrap(),
            vec![
                BranchMemo { base: "develop".to_string(), current: "feat".to_string() },
                BranchMemo { base: "feat".to_string(), current: "sub-feat".to_string() }
            ]
        );

        let _ = _add_branch_memo(&bins_dir, work_dir, s("feat"), s("sub-feat2"));

        let act = _get_branch_memos(&bins_dir, work_dir);
        assert_eq!(
            act.unwrap(),
            vec![
                BranchMemo { base: s("develop"), current: s("feat") },
                BranchMemo { base: s("feat"), current: s("sub-feat") },
                BranchMemo { base: s("feat"), current: s("sub-feat2") }
            ]
        );

        cleanup(&bins_dir);
    }

    #[test]
    fn notfound() {
        let bins_dir = PathBuf::from("target/git/test-project/memo-notfound");

        let work_dir = "/proj/2";

        let act = _get_branch_memos(&bins_dir, work_dir);
        assert_eq!(act.unwrap(), vec![]);

        let _x = _add_branch_memo(&bins_dir, work_dir, s("feat"), s("sub-feat2"));

        let act = _get_branch_memos(&bins_dir, work_dir);
        assert_eq!(act.unwrap(), vec![BranchMemo { base: s("feat"), current: s("sub-feat2") }]);

        cleanup(&bins_dir);
    }
}
