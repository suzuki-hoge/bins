use bins::fuzzy::core::item::Item;
use dialoguer::Input;
use itertools::Itertools;
use std::fmt::Debug;
use std::path::Path;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct ParsedItem {
    pub label: String,
    pub lines: Vec<String>,
    pub args: Vec<Arg>,
    pub command_type: CommandType,
}

impl ParsedItem {
    pub fn replace_args(&self) -> Vec<String> {
        let args = self
            .args
            .iter()
            .map(|arg| match &arg.default {
                Some(default) => (
                    arg.name.clone(),
                    Input::<String>::new().with_prompt(arg.name.clone()).default(default.clone()).interact().unwrap(),
                ),
                None => (arg.name.clone(), Input::<String>::new().with_prompt(arg.name.clone()).interact().unwrap()),
            })
            .collect_vec();

        let mut replaced = self.lines.clone();
        for line in &mut replaced {
            for (arg_name, arg_value) in &args {
                *line = line.replace(&format!("${}$", arg_name), arg_value);
            }
        }
        replaced
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Arg {
    pub name: String,
    pub default: Option<String>,
}

#[derive(Eq, PartialEq, Clone, Debug)]
pub enum CommandType {
    Pass,
    Run,
    Edit,
    Copy,
}

impl Item for ParsedItem {
    fn get_line(&self) -> String {
        self.label.to_string()
    }

    fn get_preview(&self) -> Vec<String> {
        self.lines.clone()
    }
}

pub trait Parser {
    fn get_items(&self, dir: &Path) -> Vec<ParsedItem>;
}
