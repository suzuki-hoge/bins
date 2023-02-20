extern crate bins;

use std::fs::File;

use itertools::Itertools;
use termion::event::Key;
use termion::get_tty;
use termion::input::TermRead;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use bins::libs::app::multi_fix_app::CursorMode::{Edit, Filter};
use bins::libs::app::multi_fix_app::{CursorMode, MultiFixApp};
use bins::libs::key::dispatcher::{
    change_cursor_mode, edit, horizontal_move, vertical_move, CURSOR_MODE_CHANGE_KEYS, EXIT_KEYS, HORIZONTAL_MOVE_KEYS,
    VERTICAL_MOVE_KEYS,
};
use bins::libs::matcher::string_matcher::MatchMode;
use bins::libs::project::project_config::parse_project_configs;

use crate::project_config::{get_project_items, ProjectItem};
use crate::ui::{draw, get_height};

pub struct Actions {
    pub cd: bool,
    pub edit: bool,
    pub github: bool,
    pub up: bool,
}

impl Actions {
    pub fn init() -> Self {
        Self { cd: false, edit: false, github: false, up: false }
    }
}

pub fn run(terminal: &mut Terminal<CrosstermBackend<File>>) -> anyhow::Result<(Vec<ProjectItem>, Actions)> {
    let project_configs = parse_project_configs()?;
    let items = get_project_items(project_configs);

    let height = get_height(&terminal.get_frame());
    let mut app = MultiFixApp::init(items, height, MatchMode::BOTH);

    let mut actions = Actions::init();
    let mut message = Ok("");
    let mut is_item_changed = false;

    let cursor_mode = app.cursor_mode.clone();
    let current_item = app.scrolling_select_app.get_active_item();
    terminal.draw(|frame| draw(frame, &mut app, get_guide(cursor_mode, current_item.clone(), &actions), message))?;

    for key in get_tty()?.keys() {
        match key.unwrap() {
            key if HORIZONTAL_MOVE_KEYS.contains(&key) && app.cursor_mode == Filter => {
                horizontal_move(&mut app, key);
                app.change_to_filter_mode();
            }
            key if VERTICAL_MOVE_KEYS.contains(&key) && app.cursor_mode == Filter => {
                vertical_move(&mut app, key);
                app.change_to_filter_mode();
                actions = Actions::init();
            }
            key if CURSOR_MODE_CHANGE_KEYS.contains(&key) => match change_cursor_mode(&mut app, key) {
                true => message = Ok(""),
                false => message = Err("can't edit"),
            },
            key if EXIT_KEYS.contains(&key) => return Ok((vec![], actions)),

            // fix one and finish
            key if key == Key::Char('\n') && app.cursor_mode == Filter => {
                app.fix();
                return Ok((app.finish(), actions));
            }

            // cd
            key if key == Key::Char('C') && app.cursor_mode == Filter => actions.cd = !actions.cd,

            // edit
            key if key == Key::Char('E') && app.cursor_mode == Filter => actions.edit = !actions.edit,

            // github
            key if key == Key::Char('G') && app.cursor_mode == Filter => {
                if let Some(item) = app.scrolling_select_app.get_active_item() {
                    if item.origin.git_exists {
                        actions.github = !actions.github
                    } else {
                        message = Err("github not found")
                    }
                }
            }

            // up
            key if key == Key::Char('U') && app.cursor_mode == Filter => {
                if let Some(item) = app.scrolling_select_app.get_active_item() {
                    if item.origin.git_exists {
                        actions.up = !actions.up
                    } else {
                        message = Err("up not found")
                    }
                }
            }

            // save
            key if key == Key::Char('S') && app.cursor_mode == Edit => {
                if let Some(mut item) = app.scrolling_select_app.get_active_item() {
                    let tags = app.edit_input_app.get();
                    item.origin.update_tags(tags);
                    is_item_changed = true;
                    message = Ok("saved");
                }
            }

            // // delete
            key if key == Key::Char('D') && app.cursor_mode == Filter => {
                if let Some(item) = app.scrolling_select_app.get_active_item() {
                    if item.origin.work_dir_exists {
                        message = Err("can't deleted");
                    } else {
                        item.origin.delete();
                        is_item_changed = true;
                        message = Ok("deleted");
                    }
                }
            }

            // edit
            key => {
                edit(&mut app, key);
                actions = Actions::init();
            }
        }

        if is_item_changed {
            let project_configs = parse_project_configs()?;
            let items = get_project_items(project_configs.clone());

            app = MultiFixApp::init(items, height, MatchMode::BOTH);
            actions = Actions::init();
        }

        let cursor_mode = app.cursor_mode.clone();
        let current_item = app.scrolling_select_app.get_active_item();
        terminal
            .draw(|frame| draw(frame, &mut app, get_guide(cursor_mode, current_item.clone(), &actions), message))?;

        message = Ok("");
        is_item_changed = false;
    }

    unreachable!();
}

fn get_guide(
    cursor_mode: CursorMode,
    current_item: Option<ProjectItem>,
    actions: &Actions,
) -> Vec<anyhow::Result<String, String>> {
    let words: Vec<anyhow::Result<String, String>> = match cursor_mode {
        Filter => vec![
            if actions.cd { Ok("C: cd") } else { Err("C: cd") },
            if actions.edit { Ok("E: edit") } else { Err("E: edit") },
            if actions.github && current_item.clone().filter(|x| x.origin.git_exists).is_some() {
                Ok("G: github")
            } else {
                Err("G: github")
            },
            if actions.up && current_item.filter(|x| x.origin.up_exists).is_some() {
                Ok("U: up")
            } else {
                Err("U: up")
            },
            Ok("D: delete"),
        ],
        Edit => vec![Ok("S: save")],
    }
    .into_iter()
    .map(|word| word.map(|s| s.to_string()).map_err(|s| s.to_string()))
    .collect_vec();
    let gap: anyhow::Result<String, String> = Ok("  |  ".to_string());
    Itertools::intersperse(words.into_iter(), gap).collect()
}
