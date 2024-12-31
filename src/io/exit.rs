use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;

pub fn exit_ok<P: AsRef<Path>, S: Into<String>>(p: P, s: S) -> anyhow::Result<ExitCode> {
    let mut file = File::create(p)?;
    writeln!(file, "{}", s.into())?;
    file.flush()?;
    Ok(ExitCode::SUCCESS)
}

pub fn exit_err<P: AsRef<Path>, S: Into<String>>(p: P, s: S) -> anyhow::Result<ExitCode> {
    let mut file = File::create(p)?;
    writeln!(file, "{}", s.into())?;
    file.flush()?;
    Ok(ExitCode::FAILURE)
}
