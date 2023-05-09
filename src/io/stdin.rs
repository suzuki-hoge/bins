use std::fmt::Display;

use std::io::Write;
use std::process;

pub fn stdout<T: Display>(value: T) -> anyhow::Result<()> {
    let r = writeln!(&mut std::io::stdout(), "{value}");
    if r.is_err() {
        process::exit(0);
    }
    Ok(())
}
