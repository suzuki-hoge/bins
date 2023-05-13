extern crate bins;

use std::collections::HashMap;
use std::string::ToString;

use bins::io::command::get_command_out_lines;
use bins::io::stdin::stdout;
use itertools::Itertools;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Opt {
    #[structopt(name = "count", default_value = "5")]
    count: u8,
    #[structopt(short = "l", long = "--long", help = "show long format")]
    long: bool,
}

#[derive(Eq, PartialEq, Debug)]
struct Logs {
    values: Vec<Log>,
}

impl Logs {
    fn from_lines(lines: Vec<String>) -> Self {
        Self { values: lines.into_iter().map(Log::new).collect_vec() }
    }

    fn short(&self) -> String {
        let mut result = String::new();

        let longest_committer_len = self.get_longest_committer_len();
        let groups: HashMap<String, Vec<&Log>> = self.values.iter().into_group_map_by(|log| log.short_date.clone());
        let mut groups: Vec<(&String, &Vec<&Log>)> = groups.iter().collect();
        groups.sort_by_key(|(date, _)| date.to_string());
        groups.reverse();
        for (date, logs) in groups {
            result = format!("{result}\n{date}");
            for log in logs {
                result = format!("{}\n{}", result, log.short(longest_committer_len));
            }
            result = format!("{result}\n");
        }

        result.trim().to_string()
    }

    fn long(&self) -> String {
        self.values.iter().map(|log| log.long(self.get_longest_committer_len())).join("\n")
    }

    fn get_longest_committer_len(&self) -> usize {
        self.values.iter().map(|log| log.committer.len()).max().unwrap_or(0)
    }
}

#[derive(Eq, PartialEq, Debug)]
struct Log {
    hash: String,
    committer: String,
    short_date: String,
    long_date: String,
    message: String,
}

impl Log {
    fn new(line: String) -> Self {
        let sp = line.split("|--|").collect_vec();
        let hash = sp[0].to_string();
        let committer = sp[1].to_string();
        let short_date = sp[2].split(' ').collect_vec()[0].to_string();
        let long_date = sp[2].to_string();
        let message = sp[3].to_string();
        Self { hash, committer, short_date, long_date, message }
    }

    fn short(&self, longest_committer_len: usize) -> String {
        let pad = " ".repeat(longest_committer_len - self.committer.len());
        format!("  {}{} - {}", Self::blue(&self.committer), pad, self.message)
    }

    fn long(&self, longest_committer_len: usize) -> String {
        let pad = " ".repeat(longest_committer_len - self.committer.len());
        format!(
            "{} [ {} ] {}{} {}",
            Self::yellow(&self.hash),
            self.long_date,
            Self::blue(&self.committer),
            pad,
            self.message
        )
    }

    fn blue(s: &str) -> String {
        format!("\x1b[34m{s}\x1b[m")
    }

    fn yellow(s: &str) -> String {
        format!("\x1b[33m{s}\x1b[m")
    }
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let command = create_command(opt.count);
    let lines = get_command_out_lines(command)?;
    let logs = Logs::from_lines(lines);

    match opt.long {
        true => stdout(logs.long()),
        false => stdout(logs.short()),
    }
}

fn create_command(count: u8) -> String {
    format!("git log -{count} '--date=format:%Y/%m/%d %H:%M:%S' '--pretty=%h|--|%cn|--|%cd|--|%s'")
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;
    use trim_margin::MarginTrimmable;

    use crate::Logs;

    #[test]
    fn test() {
        let raw = "
            |93f6b29|--|GitHub|--|2023/02/04 10:59:27|--|Merge pull request #50 from feat
            |80d69b0|--|suzuki-hoge|--|2023/02/03 11:15:40|--|Merge pull request #49 from feat/foo
            |4a7ef5d|--|john-doe|--|2023/02/03 11:15:39|--|Merge pull request #48 from feat/bar
            |a7e013e|--|john-doe-bot|--|2023/02/02 15:42:10|--|auto fix format.
        "
        .trim()
        .trim_margin()
        .unwrap()
        .split('\n')
        .into_iter()
        .map(|s| s.to_string())
        .collect_vec();

        let logs = Logs::from_lines(raw);

        let short = "
            |2023/02/04
            |  \x1b[34mGitHub\x1b[m       - Merge pull request #50 from feat
            |
            |2023/02/03
            |  \x1b[34msuzuki-hoge\x1b[m  - Merge pull request #49 from feat/foo
            |  \x1b[34mjohn-doe\x1b[m     - Merge pull request #48 from feat/bar
            |
            |2023/02/02
            |  \x1b[34mjohn-doe-bot\x1b[m - auto fix format.
        "
        .trim()
        .trim_margin()
        .unwrap();

        let long = "
            |\x1b[33m93f6b29\x1b[m [ 2023/02/04 10:59:27 ] \x1b[34mGitHub\x1b[m       Merge pull request #50 from feat
            |\x1b[33m80d69b0\x1b[m [ 2023/02/03 11:15:40 ] \x1b[34msuzuki-hoge\x1b[m  Merge pull request #49 from feat/foo
            |\x1b[33m4a7ef5d\x1b[m [ 2023/02/03 11:15:39 ] \x1b[34mjohn-doe\x1b[m     Merge pull request #48 from feat/bar
            |\x1b[33ma7e013e\x1b[m [ 2023/02/02 15:42:10 ] \x1b[34mjohn-doe-bot\x1b[m auto fix format.
        "
            .trim()
            .trim_margin()
            .unwrap();

        assert_eq!(logs.short(), short);
        assert_eq!(logs.long(), long);
    }
}
