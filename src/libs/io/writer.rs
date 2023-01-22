use std::fmt::Display;
use std::io::stdout;
use std::io::Write;
use std::process;

pub fn output_or_exit<T: Display>(value: T) -> anyhow::Result<()> {
    let r = writeln!(&mut stdout(), "{}", value);
    if r.is_err() {
        process::exit(0);
    }
    Ok(())
}
