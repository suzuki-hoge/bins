use std::process::Command;
use std::process::Stdio;

use anyhow::{anyhow, Context};
use futures::prelude::*;
use itertools::Itertools;
use tokio_util::codec::{FramedRead, LinesCodec};

use crate::libs::io::writer::output_or_exit;

pub async fn print_command_out(s: impl Into<String>) -> anyhow::Result<()> {
    let s = s.into();
    let (name, args) = parse_command(&s);

    let mut child = tokio::process::Command::new(name)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()
        .context(format!("failed tokio process spawn. [ {} ]", &s))?;

    let stdout = child.stdout.as_mut().unwrap();
    let mut reader = FramedRead::new(stdout, LinesCodec::new());

    while let Some(line) = reader.next().await {
        let _ = output_or_exit(line.unwrap());
    }

    Ok(())
}

pub fn get_command_out_lines(s: impl Into<String>) -> anyhow::Result<Vec<String>> {
    let s = s.into();
    let (name, args) = parse_command(&s);

    let output = Command::new(name).args(args).output().context(format!("failed std process create. [ {} ]", &s))?;

    let out = String::from_utf8(output.stdout).expect("Not UTF-8");
    let err = String::from_utf8(output.stderr).expect("Not UTF-8");

    if output.status.success() {
        let lines = if err.is_empty() { out } else { err }; // care stderr exists on success ( e.g. git checkout )
        Ok(lines.trim_end().split('\n').into_iter().map(|s| s.to_string()).collect_vec())
    } else {
        Err(anyhow!(err))
    }
}

pub fn get_command_out_line(s: impl Into<String>) -> anyhow::Result<String> {
    let lines = get_command_out_lines(s)?;

    if lines.is_empty() {
        Ok(String::new())
    } else {
        Ok(lines[0].to_string())
    }
}

pub fn run_command(s: impl Into<String>) -> anyhow::Result<()> {
    get_command_out_lines(s)?;
    Ok(())
}

fn parse_command(s: &str) -> (String, Vec<String>) {
    let mut words = vec![];
    let mut word = String::new();
    let mut in_quote = false;
    for c in s.chars() {
        if c == ' ' && !in_quote {
            words.push(word.clone());
            word = String::new();
        } else if c == '\'' {
            in_quote = !in_quote
        } else {
            word = format!("{word}{c}");
        }
    }
    words.push(word);

    let head = words[0].to_string();
    let init = words.into_iter().skip(1).collect_vec();
    (head, init)
}

#[cfg(test)]
mod tests {
    use crate::libs::process::command::{get_command_out_lines, parse_command};

    #[test]
    fn test_parse() {
        assert_eq!(parse_command("ls"), ("ls".to_string(), vec![]));
        assert_eq!(
            parse_command("git status --short --branch"),
            ("git".to_string(), vec!["status".to_string(), "--short".to_string(), "--branch".to_string()])
        );
        assert_eq!(
            parse_command("git commit --message 'foo bar'"),
            ("git".to_string(), vec!["commit".to_string(), "--message".to_string(), "foo bar".to_string()])
        );
    }

    #[test]
    fn test_get_command_out() {
        // ok
        let out = get_command_out_lines("find . -name Cargo*");
        assert!(out.is_ok());
        assert_eq!(out.unwrap(), vec![format!("./Cargo.toml"), format!("./Cargo.lock")]);

        // command error
        let out = get_command_out_lines("foo");
        assert!(out.is_err());
        assert_eq!(out.err().unwrap().to_string(), format!("failed std process create. [ foo ]"));

        // args error
        let out = get_command_out_lines("ls -2");
        assert!(out.is_err());
        assert!(out.err().unwrap().to_string().contains("invalid option -- 2"));
    }
}
