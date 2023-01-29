extern crate bins;

use bins::libs::git::branch::{get_git_branch, GitBranch};
use bins::libs::git::config::get_git_config;
use bins::libs::git::file::get_git_paths;
use itertools::Itertools;
use std::env::current_dir;
use std::path::PathBuf;

use crate::url_item::UrlItems;
use bins::libs::io::writer::output_or_exit;
use bins::libs::launcher::crossterm_launcher::launch;

mod runner;
mod ui;
mod url_item;

fn main() -> anyhow::Result<()> {
    let home = PathBuf::from(std::env::var("HOME")?);
    let dir_path = current_dir()?;

    let git_config = get_git_config()?;
    let git_branch = get_git_branch(&home, &dir_path)?;

    let mut url_items = UrlItems::create(&git_config);

    gather_pulls(&mut url_items);
    gather_issues(&mut url_items);
    gather_tree(&mut url_items, &git_branch);
    gather_commits(&mut url_items, &git_branch);
    gather_compare(&mut url_items, &git_branch);
    gather_wiki(&mut url_items);
    gather_find(&mut url_items, &git_branch);
    gather_blob(&mut url_items, &git_branch);

    match launch(|terminal| runner::run(terminal, url_items.get_items())) {
        Ok(items) => output_or_exit(items.iter().map(|item| item.get_raw()).join("\n")),
        Err(e) => output_or_exit(format!("echo {}", e)),
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

fn gather_tree(url_items: &mut UrlItems, branch: &GitBranch) {
    branch.get_all().iter().for_each(|branch| url_items.add(format!("files - {}", branch), format!("tree/{}", branch)));
}

fn gather_commits(url_items: &mut UrlItems, branch: &GitBranch) {
    branch
        .get_all()
        .iter()
        .for_each(|branch| url_items.add(format!("commits - {}", branch), format!("commits/{}", branch)));
}

fn gather_compare(url_items: &mut UrlItems, branch: &GitBranch) {
    if let Some((base, current)) = branch.get_compare() {
        url_items.add("pr", format!("compare/{}...{}", base, current));
    }
}

fn gather_wiki(url_items: &mut UrlItems) {
    url_items.add("wiki", "wiki");
}

fn gather_find(url_items: &mut UrlItems, branch: &GitBranch) {
    branch.get_all().iter().for_each(|branch| url_items.add(format!("find - {}", branch), format!("find/{}", branch)));
}

fn gather_blob(url_items: &mut UrlItems, branch: &GitBranch) {
    let paths = get_git_paths();
    branch.get_all().iter().for_each(|branch| {
        paths
            .iter()
            .for_each(|path| url_items.add(format!("{} - {}", path, branch), format!("blob/{}/{}", branch, path)))
    });
}
