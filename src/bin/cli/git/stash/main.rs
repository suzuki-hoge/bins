extern crate bins;

use structopt::StructOpt;

use bins::libs::process::command::print_command_out;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "l", long = "--list", conflicts_with_all(& ["save", "pop"]), help = "show stash list ( default )")]
    list: bool,

    #[structopt(short = "s", long = "--save", conflicts_with_all(& ["list", "pop"]), help = "stash")]
    save: bool,

    #[structopt(short = "p", long = "--pop", conflicts_with_all(& ["list", "save"]), help = "pop")]
    pop: bool,

    #[structopt(name = "message", help = "message for save")]
    message: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();
    print_command_out(create_command(opt.list, opt.save, opt.pop, opt.message.as_deref()))
}

fn create_command(list: bool, save: bool, pop: bool, message: Option<&str>) -> String {
    match (list, save, pop, message) {
        (_, true, _, Some(message)) => format!("git stash save '{message}'"),
        (_, true, _, None) => "git stash save".to_string(),
        (_, _, true, _) => "git stash pop".to_string(),
        (_, _, _, _) => "git stash list".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use crate::create_command;

    #[test]
    fn test_create_command() {
        assert_eq!(create_command(true, false, false, None), "git stash list".to_string());
        assert_eq!(create_command(false, true, false, Some("foo bar")), "git stash save 'foo bar'".to_string());
        assert_eq!(create_command(false, true, false, None), "git stash save".to_string());
        assert_eq!(create_command(false, false, true, None), "git stash pop".to_string());
    }
}
