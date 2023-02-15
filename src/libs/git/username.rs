use crate::libs::process::command::get_command_out_line;

pub fn get_git_username() -> anyhow::Result<String> {
    get_command_out_line("git config --global user.name")
}
