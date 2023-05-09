use crate::gatherer::get_piped_stdin_or_dummy;
use bins::fuzzy::FuzzyBuilder;
use bins::io::stdin::stdout;
use itertools::Itertools;

mod gatherer;

fn main() -> anyhow::Result<()> {
    let items = get_piped_stdin_or_dummy()?;

    let (items, _) = FuzzyBuilder::simple(items).build().run()?;

    stdout(items.into_iter().join("\n"))
}
