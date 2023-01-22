use std::fs::{remove_file, File};
use std::io::{Read, Write};
use std::path::Path;
use std::{fs, io};

use itertools::Itertools;
use trim_margin::MarginTrimmable;

struct Readme {
    dir_path: String,
    name: String,
    description: String,
}

impl Readme {
    fn new<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let dir_path = path.as_ref().display().to_string().split('/').take_while(|&s| s != "README.md").join("/");
        let head_line = read_head_line(path)?;
        let name = head_line.split('(').collect_vec()[1].split(')').collect_vec()[0].trim().to_string();
        let description = head_line.split('#').collect_vec()[1].split('(').collect_vec()[0].trim().to_string();
        Ok(Readme { dir_path, name, description })
    }

    fn to_line(&self) -> String {
        format!("- [{}: {}]({})", self.name, self.description, self.dir_path)
    }
}

fn main() -> io::Result<()> {
    println!("\ngather readme.");

    if Path::new("README.md").exists() {
        let _ = remove_file("README.md");
    }
    let mut file = File::options().create(true).write(true).open("README.md").unwrap();

    let template = "
            |# Bins
            |My commands.
            |
            |## Command list
            |
        "
    .trim()
    .trim_margin()
    .unwrap();
    write!(file, "{}", template)?;

    process(&mut file, "tui", &["project", "git", "misc"])?;
    process(&mut file, "cli", &["git"])?;

    file.flush().unwrap();

    Ok(())
}

fn process(file: &mut File, dir: &str, sub_dirs: &[&str]) -> io::Result<()> {
    writeln!(file, "### {}", dir)?;

    for sub_dir in sub_dirs {
        writeln!(file, "#### {}", sub_dir)?;

        for readme in search(format!("src/bin/{}/{}", dir, sub_dir))? {
            writeln!(file, "{}", readme.to_line())?;
        }
    }

    Ok(())
}

fn search<P: AsRef<Path>>(path: P) -> io::Result<Vec<Readme>> {
    let mut acc = vec![];
    visit_dir(path, &mut acc)?;
    Ok(acc)
}

fn visit_dir<P: AsRef<Path>>(path: P, acc: &mut Vec<Readme>) -> io::Result<()> {
    let mut es = fs::read_dir(path)?.map(|e| e.unwrap()).collect_vec();
    es.sort_by_key(|e| e.path());
    for e in es {
        if e.file_type()?.is_dir() {
            visit_dir(e.path(), acc)?;
        }
        if e.path().ends_with("README.md") {
            acc.push(Readme::new(e.path())?);
        }
    }
    Ok(())
}

fn read_head_line<P: AsRef<Path>>(path: P) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut lines = String::new();
    file.read_to_string(&mut lines)?;
    Ok(lines.split('\n').collect_vec()[0].to_string())
}
