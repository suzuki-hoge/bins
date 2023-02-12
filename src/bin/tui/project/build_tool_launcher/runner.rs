extern crate bins;

use std::fs::File;

use itertools::Itertools;
use regex::Regex;
use termion::event::Key;
use termion::get_tty;
use termion::input::TermRead;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use bins::libs::app::multi_fix_app::CursorMode::{Edit, Filter};
use bins::libs::app::multi_fix_app::{CursorMode, MultiFixApp};
use bins::libs::item::display_item::DisplayItem;

use bins::libs::key::dispatcher::{
    change_cursor_mode, edit, exit, horizontal_move, vertical_move, CURSOR_MODE_CHANGE_KEYS, EXIT_KEYS,
    HORIZONTAL_MOVE_KEYS, VERTICAL_MOVE_KEYS,
};
use bins::libs::matcher::string_matcher::MatchMode;
use bins::libs::project::project_config::ProjectConfig;

use crate::command::command_item::CommandItem;
use crate::command::project_config::get_command_items;
use crate::ui::{draw, get_height};

pub fn run(
    terminal: &mut Terminal<CrosstermBackend<File>>,
    mut project_config: ProjectConfig,
    freezed_items: Vec<CommandItem>,
) -> anyhow::Result<Vec<CommandItem>> {
    let mut items = freezed_items.clone();
    items.append(get_command_items(&project_config).as_mut());

    let height = get_height(&terminal.get_frame());
    let mut app = MultiFixApp::init(items, height, MatchMode::BOTH);

    let mut message = Ok("");
    let mut is_item_changed = false;

    let cursor_mode = app.cursor_mode.clone();
    terminal.draw(|frame| draw(frame, &mut app, get_guide(cursor_mode), message))?;

    for key in get_tty()?.keys() {
        match key.unwrap() {
            key if HORIZONTAL_MOVE_KEYS.contains(&key) && app.cursor_mode == Filter => {
                horizontal_move(&mut app, key);
                app.change_to_filter_mode();
            }
            key if VERTICAL_MOVE_KEYS.contains(&key) && app.cursor_mode == Filter => {
                vertical_move(&mut app, key);
                app.change_to_filter_mode();
            }
            key if CURSOR_MODE_CHANGE_KEYS.contains(&key) => match change_cursor_mode(&mut app, key) {
                true => message = Ok(""),
                false => message = Err("can't edit"),
            },
            key if EXIT_KEYS.contains(&key) => return exit(&mut app, key),

            // fix one
            key if key == Key::Null && app.cursor_mode == Filter => app.fix(),

            // finish
            key if key == Key::Ctrl('f') && app.cursor_mode == Filter => return Ok(app.finish()),

            // fix one and finish
            key if key == Key::Char('\n') && app.cursor_mode == Filter => {
                app.fix();
                return Ok(app.finish());
            }

            // new
            key if key == Key::Char('N') && app.cursor_mode == Filter => {
                let new_label = app.filter_input_app.get().join("");
                let new_lines = vec![];

                if Regex::new(r"^[a-z]+$").unwrap().is_match(&new_label) {
                    project_config.upsert_build_command(new_label, new_lines);

                    is_item_changed = true;
                    message = Ok("created");
                } else {
                    message = Err("invalid new label")
                }
            }

            // save
            key if key == Key::Char('S') && app.cursor_mode == Edit => {
                if let Some(item) = app.scrolling_select_app.get_active_item() {
                    let label = item.get_pane1();
                    let new_lines = app.edit_input_app.get();
                    project_config.upsert_build_command(label, new_lines);

                    is_item_changed = true;
                    message = Ok("saved");
                }
            }

            // delete
            key if key == Key::Char('D') && app.cursor_mode == Filter => {
                if let Some(item) = app.scrolling_select_app.get_active_item() {
                    if item.is_editable() {
                        let label = item.get_pane1();
                        project_config.delete_build_command(label);

                        is_item_changed = true;
                        message = Ok("deleted");
                    } else {
                        message = Err("can't delete");
                    }
                }
            }

            // edit
            key => edit(&mut app, key),
        }

        if is_item_changed {
            let mut items = freezed_items.clone();
            items.append(get_command_items(&project_config).as_mut());

            app = MultiFixApp::init(items, height, MatchMode::BOTH);
        }

        let cursor_mode = app.cursor_mode.clone();
        terminal.draw(|frame| draw(frame, &mut app, get_guide(cursor_mode), message))?;

        message = Ok("");
        is_item_changed = false;
    }

    unreachable!();
}

fn get_guide(cursor_mode: CursorMode) -> Vec<anyhow::Result<String, String>> {
    let words: Vec<anyhow::Result<String, String>> = match cursor_mode {
        Filter => vec![Ok("N: new"), Ok("D: delete")],
        Edit => vec![Ok("S: save")],
    }
    .into_iter()
    .map(|word| word.map(|s| s.to_string()))
    .collect_vec();
    let gap: anyhow::Result<String, String> = Ok(" | ".to_string());
    Itertools::intersperse(words.into_iter(), gap).collect()
}
