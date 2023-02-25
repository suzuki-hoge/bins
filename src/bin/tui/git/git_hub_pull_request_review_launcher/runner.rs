extern crate bins;

use itertools::Itertools;
use std::fs::File;

use termion::event::Key;
use termion::get_tty;
use termion::input::TermRead;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use bins::libs::app::multi_fix_app::MultiFixApp;
use bins::libs::key::dispatcher::{
    edit, horizontal_move, vertical_move, EXIT_KEYS, HORIZONTAL_MOVE_KEYS, VERTICAL_MOVE_KEYS,
};
use bins::libs::matcher::string_matcher::MatchMode;
use ui::draw;

use crate::pull_request::PullRequest;
use crate::ui;
use crate::ui::get_height;

pub struct Actions {
    pub switch: bool,
    pub git_hub: bool,
}

impl Actions {
    pub fn init() -> Self {
        Self { switch: true, git_hub: false }
    }
}

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<File>>,
    pull_requests: Vec<PullRequest>,
    username: &String,
) -> anyhow::Result<(Vec<PullRequest>, Actions)> {
    let height = get_height(&terminal.get_frame());
    let mut current_tab = 1;

    let mut app = create_app(height, current_tab, pull_requests.clone(), username);

    let mut actions = Actions::init();

    terminal.draw(|frame| draw(frame, current_tab, &app, get_guide(&actions)))?;

    for key in get_tty()?.keys() {
        match key.unwrap() {
            key if HORIZONTAL_MOVE_KEYS.contains(&key) => horizontal_move(&mut app, key),
            key if VERTICAL_MOVE_KEYS.contains(&key) => vertical_move(&mut app, key),
            key if EXIT_KEYS.contains(&key) => return Ok((vec![], actions)),

            // change tab
            Key::Char('\t') => {
                current_tab = next_tab(current_tab);
                app = create_app(height, current_tab, pull_requests.clone(), username);
            }
            Key::BackTab => {
                current_tab = prev_tab(current_tab);
                app = create_app(height, current_tab, pull_requests.clone(), username);
            }

            // actions
            key if key == Key::Char('S') => actions.switch = !actions.switch,

            key if key == Key::Char('G') => actions.git_hub = !actions.git_hub,

            // fix one and finish
            Key::Char('\n') => {
                app.fix();
                return Ok((app.finish(), actions));
            }

            key => edit(&mut app, key),
        }

        terminal.draw(|frame| draw(frame, current_tab, &app, get_guide(&actions)))?;
    }

    unreachable!();
}

fn create_app(
    height: u16,
    current_tab: usize,
    pull_requests: Vec<PullRequest>,
    username: &String,
) -> MultiFixApp<PullRequest> {
    let items = match current_tab {
        1 => pull_requests.into_iter().collect_vec(),
        2 => pull_requests.into_iter().filter(|pr| pr.is_own(username)).collect_vec(),
        3 => pull_requests.into_iter().filter(|pr| pr.is_not_reviewed(username)).collect_vec(),
        4 => pull_requests.into_iter().filter(|pr| pr.is_reviewed(username)).collect_vec(),
        _ => unreachable!(),
    };
    MultiFixApp::init(items, height, MatchMode::PANE1)
}

fn next_tab(current_tab: usize) -> usize {
    match current_tab {
        4 => 1,
        n => n + 1,
    }
}
fn prev_tab(current_tab: usize) -> usize {
    match current_tab {
        1 => 4,
        n => n - 1,
    }
}

fn get_guide(actions: &Actions) -> Vec<anyhow::Result<String, String>> {
    let words: Vec<anyhow::Result<String, String>> = vec![
        if actions.switch { Ok("S: switch") } else { Err("S: switch") },
        if actions.git_hub { Ok("G: github") } else { Err("G: github") },
    ]
    .into_iter()
    .map(|word| word.map(|s| s.to_string()).map_err(|s| s.to_string()))
    .collect_vec();
    let gap: anyhow::Result<String, String> = Ok("  |  ".to_string());
    Itertools::intersperse(words.into_iter(), gap).collect()
}
