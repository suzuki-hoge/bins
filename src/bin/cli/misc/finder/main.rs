use std::env::current_dir;
use std::fs::read_dir;
use std::path::{Path, PathBuf};

use structopt::StructOpt;

use crate::ignore::{get_ignores, Ignore};
use crate::Target::{Directory, File};

mod ignore;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "-f", long = "--file", conflicts_with_all(& ["directory"]), help = "search file")]
    file: bool,
    #[structopt(short = "-d", long = "--directory", conflicts_with_all(& ["file"]), help = "search directory")]
    directory: bool,
    #[structopt(short = "-r", long = "--recursive", help = "search recursively")]
    recursive: bool,
    #[structopt(name = "path")]
    path: Option<String>,
}

#[derive(Eq, PartialEq)]
enum Target {
    File,
    Directory,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let path = opt.path.map(|s| PathBuf::from(s)).unwrap_or(current_dir()?);
    let root_len = path.display().to_string().len();
    let target = match (opt.file, opt.directory) {
        (true, false) => File,
        (false, true) => Directory,
        (_, _) => unreachable!(),
    };

    let ignore = get_ignores()?;

    find(&path, root_len, &target, opt.recursive, &ignore)
}

fn find(path: &Path, root_len: usize, target: &Target, recursive: bool, ignore: &Ignore) -> anyhow::Result<()> {
    for e in read_dir(path)? {
        let e = e?;

        if e.file_type()?.is_file() {
            if target == &File {
                println!("{}", &e.path().display().to_string()[root_len + 1..]);
            }
        } else {
            if target == &Directory {
                println!("{}", &e.path().display().to_string()[root_len + 1..]);
            }
            if recursive && !ignore.directories.contains(&e.file_name().into_string().unwrap()) {
                find(&e.path(), root_len, target, recursive, ignore)?
            }
        }
    }
    Ok(())
}
