extern crate bins;

use structopt::StructOpt;


use bins::io::stdin::stdout;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "s", long = "--staged", help = "show staged diff")]
    staged: bool,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    match opt.staged {
        true => stdout("git diff --staged"),
        false => stdout("git diff"),
    }
}
