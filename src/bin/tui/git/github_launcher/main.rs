extern crate bins;

use bins::libs::git::branch::{get_git_branch, GitBranch};
use bins::libs::git::config::get_git_config;
use itertools::Itertools;

use crate::url_item::UrlItems;
use bins::libs::io::writer::output_or_exit;
use bins::libs::launcher::crossterm_launcher::launch;
use bins::libs::process::command::get_command_out_lines;

mod runner;
mod ui;
mod url_item;

fn main() -> anyhow::Result<()> {
    let args = std::env::args().collect_vec();

    let git_config = get_git_config()?;
    let git_branch = get_git_branch()?;

    let mut url_items = UrlItems::create(&git_config);

    if args.len() == 1 {
        gather(&git_branch, url_items)
    } else if args.len() == 2 {
        let current = git_branch.current;

        let out = match args[1].as_str() {
            "p" => url_items.add("pulls", "pulls").get_raw(),
            "i" => url_items.add("issues", "issues").get_raw(),
            "f" => url_items.add(format!("files - {current}"), format!("tree/{current}")).get_raw(),
            "t" => url_items.add(format!("find - {current}"), format!("find/{current}")).get_raw(),
            _ => "echo no such option".to_string(),
        };
        output_or_exit(out)
    } else {
        output_or_exit("echo invalid args")
    }
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
        Ok(items) => output_or_exit(items.iter().map(|item| item.get_raw()).join("\n")),
        Err(e) => output_or_exit(format!("echo {e}")),
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
