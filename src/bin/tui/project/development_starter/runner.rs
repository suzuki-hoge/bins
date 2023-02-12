extern crate bins;

use std::fs::File;

use termion::event::Key;
use termion::get_tty;
use termion::input::TermRead;
use tui::backend::CrosstermBackend;
use tui::Terminal;

use bins::libs::app::multi_fix_app::CursorMode::{Edit, Filter};
use bins::libs::app::multi_fix_app::{CursorMode, MultiFixApp};

use bins::libs::key::dispatcher::CommandMode::{Active, Inactive};
use bins::libs::key::dispatcher::{
    change_cursor_mode, edit, horizontal_move, vertical_move, CommandMode, CHANGE_COMMAND_MODE_KEYS,
    CURSOR_MODE_CHANGE_KEYS, EXIT_KEYS, HORIZONTAL_MOVE_KEYS, VERTICAL_MOVE_KEYS,
};
use bins::libs::matcher::string_matcher::MatchMode;
use bins::libs::project::project_config::parse_project_configs;

use crate::project_config::get_project_items;
use crate::ui::{draw, get_height};

pub fn run(terminal: &mut Terminal<CrosstermBackend<File>>) -> anyhow::Result<()> {
    let project_configs = parse_project_configs()?;
    let items = get_project_items(project_configs);

    let height = get_height(&terminal.get_frame());
    let mut app = MultiFixApp::init(items, height, MatchMode::BOTH);

    let mut message = Ok("");
    let mut command_mode = Inactive;
    let mut is_item_changed = false;

    let cursor_mode = app.cursor_mode.clone();
    terminal.draw(|frame| draw(frame, &mut app, get_guide(&command_mode, cursor_mode), message))?;

    for key in get_tty()?.keys() {
        match command_mode {
            Inactive => {
                match key.unwrap() {
                    key if HORIZONTAL_MOVE_KEYS.contains(&key) && app.cursor_mode == Filter => {
                        horizontal_move(&mut app, key);
                        app.change_to_filter_mode();
                    }
                    key if VERTICAL_MOVE_KEYS.contains(&key) && app.cursor_mode == Filter => {
                        vertical_move(&mut app, key);
                        app.change_to_filter_mode();
                    }
                    key if CHANGE_COMMAND_MODE_KEYS.contains(&key) => command_mode = Active,
                    key if CURSOR_MODE_CHANGE_KEYS.contains(&key) => match change_cursor_mode(&mut app, key) {
                        true => message = Ok(""),
                        false => message = Err("can't edit"),
                    },
                    key if EXIT_KEYS.contains(&key) => return Ok(()),

                    // fix one
                    key if key == Key::Null && app.cursor_mode == Filter => app.fix(),

                    // finish
                    key if key == Key::Ctrl('f') && app.cursor_mode == Filter => return Ok(()),

                    // fix one and finish
                    key if key == Key::Char('\n') && app.cursor_mode == Filter => {
                        app.fix();
                        return Ok(());
                    }

                    key => edit(&mut app, key),
                }
            }
            Active => {
                match key.unwrap() {
                    key if CHANGE_COMMAND_MODE_KEYS.contains(&key) => command_mode = Inactive,

                    // save
                    key if key == Key::Char('s') && app.cursor_mode == Edit => {
                        if let Some(mut item) = app.scrolling_select_app.get_active_item() {
                            let tags = app.edit_input_app.get();
                            item.origin.update_tags(tags);
                            is_item_changed = true;
                            message = Ok("saved");
                        }
                    }

                    // // delete
                    key if key == Key::Char('d') && app.cursor_mode == Filter => {
                        if let Some(mut item) = app.scrolling_select_app.get_active_item() {
                            item.origin.delete();
                            is_item_changed = true;
                            message = Ok("deleted");
                        }
                    }

                    _ => command_mode = Inactive,
                }
            }
        }

        if is_item_changed {
            let project_configs = parse_project_configs()?;
            let items = get_project_items(project_configs.clone());

            app = MultiFixApp::init(items, height, MatchMode::BOTH);

            command_mode = Inactive;
        }

        let cursor_mode = app.cursor_mode.clone();
        terminal.draw(|frame| draw(frame, &mut app, get_guide(&command_mode, cursor_mode), message))?;

        message = Ok("");
        is_item_changed = false;
    }

    unreachable!();
}

fn get_guide(key_mode: &CommandMode, cursor_mode: CursorMode) -> &str {
    match (key_mode, cursor_mode) {
        (Inactive, _) => "",
        (Active, Filter) => "n: new | d: delete",
        (Active, Edit) => "s: save",
    }
}
