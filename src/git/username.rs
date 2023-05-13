use crate::io::command::get_command_out_line;

pub fn get_git_username() -> String {
    get_command_out_line("git config --global user.name").unwrap()
}
