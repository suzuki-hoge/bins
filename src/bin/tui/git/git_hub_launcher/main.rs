extern crate bins;

use structopt::StructOpt;

use bins::libs::git::branch::{get_git_branch, GitBranch};
use bins::libs::git::config::get_git_config;
use bins::libs::io::writer::stderr;
use bins::libs::launcher::crossterm_launcher::launch;
use bins::libs::process::command::{get_command_out_lines, run_command};

use crate::url_item::UrlItems;

mod runner;
mod ui;
mod url_item;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "p", long = "--pulls", conflicts_with_all(& ["issues", "files", "tree"]), help = "open pulls page")]
    pulls: bool,

    #[structopt(short = "i", long = "--issues", conflicts_with_all(& ["pulls", "files", "tree"]), help = "open issues page")]
    issues: bool,

    #[structopt(short = "f", long = "--files", conflicts_with_all(& ["pulls", "issues", "tree"]), help = "open files page")]
    files: bool,

    #[structopt(short = "t", long = "--tree", conflicts_with_all(& ["pulls", "issues", "files"]), help = "open tree page")]
    tree: bool,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let git_config = get_git_config()?;
    let git_branch = get_git_branch()?;

    let mut url_items = UrlItems::create(&git_config);

    if !opt.pulls && !opt.issues && !opt.files && !opt.tree {
        gather(&git_branch, url_items)
    } else {
        let current = git_branch.current;

        if opt.pulls {
            open(url_items.add("pulls", "pulls").get_raw())
        } else if opt.issues {
            open(url_items.add("issues", "issues").get_raw())
        } else if opt.files {
            open(url_items.add(format!("files - {current}"), format!("tree/{current}")).get_raw())
        } else {
            open(url_items.add(format!("find - {current}"), format!("find/{current}")).get_raw())
        }
    }
}

fn open(url: String) -> anyhow::Result<()> {
    run_command(format!("open {url}"))
}

fn gather(branch: &GitBranch, mut url_items: UrlItems) -> anyhow::Result<()> {
    gather_pulls(&mut url_items);
    gather_issues(&mut url_items);
    gather_tree(&mut url_items, &branch.current, branch.base.as_deref());
    gather_commits(&mut url_items, &branch.current, branch.base.as_deref());
    gather_compare(&mut url_items, &branch.current, branch.base.as_deref());
    gather_wiki(&mut url_items);
    gather_find(&mut url_items, &branch.current, branch.base.as_deref());
    gather_blob(&mut url_items, &branch.current, branch.base.as_deref())?;

    match launch(|terminal| runner::run(terminal, url_items.get_items())) {
        Ok(items) => {
            for item in items {
                let _ = open(item.get_raw());
            }
            Ok(())
        }
        Err(e) => stderr(e),
    }
}

fn gather_pulls(url_items: &mut UrlItems) {
    url_items.add("pulls", "pulls");
    url_items.add("my pulls", "pulls/@me");
    url_items.add("review pulls", "pulls?q=is:open+is:pr+-reviewed-by:@me+reviewed-by:@me");
}

fn gather_issues(url_items: &mut UrlItems) {
    url_items.add("issues", "issues");
}

fn gather_tree(url_items: &mut UrlItems, current: &str, base: Option<&str>) {
    url_items.add(format!("files - {current}"), format!("tree/{current}"));
    if let Some(base) = base {
        url_items.add(format!("files - {base}"), format!("tree/{base}"));
    }
}

fn gather_commits(url_items: &mut UrlItems, current: &str, base: Option<&str>) {
    url_items.add(format!("commits - {current}"), format!("commits/{current}"));
    if let Some(base) = base {
        url_items.add(format!("commits - {base}"), format!("commits/{base}"));
    }
}

fn gather_compare(url_items: &mut UrlItems, current: &str, base: Option<&str>) {
    if let Some(base) = base {
        url_items.add("pr".to_string(), format!("compare/{base}...{current}"));
    }
}

fn gather_wiki(url_items: &mut UrlItems) {
    url_items.add("wiki", "wiki");
}

fn gather_find(url_items: &mut UrlItems, current: &str, base: Option<&str>) {
    url_items.add(format!("find - {current}"), format!("find/{current}"));
    if let Some(base) = base {
        url_items.add(format!("find - {base}"), format!("find/{base}"));
    }
}

fn gather_blob(url_items: &mut UrlItems, current: &str, base: Option<&str>) -> anyhow::Result<()> {
    for path in get_command_out_lines("git ls-files")? {
        url_items.add(format!("{path} - {current}"), format!("blob/{current}/{path}"));
        if let Some(base) = base {
            url_items.add(format!("{path} - {base}"), format!("blob/{base}/{path}"));
        }
    }
    Ok(())
}
