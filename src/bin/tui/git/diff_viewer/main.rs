use bins::fuzzy::core::style::CustomPreviewStyle;
use bins::fuzzy::FuzzyBuilder;

use structopt::StructOpt;
use tui::layout::Constraint::Percentage;
use tui::layout::Direction::Horizontal;
use tui::style::{Color, Style};
use tui::text::Span;
use tui::widgets::ListItem;

use crate::item::gather_diffs;

mod item;

#[derive(StructOpt)]
struct Opt {
    #[structopt(short = "s", long = "--staged", help = "show staged diff")]
    staged: bool,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let items = gather_diffs(opt.staged);

    FuzzyBuilder::pane(items, Horizontal, Percentage(30)).custom_preview(Diff {}).build().run().map(|_| ())
}

struct Diff {}

impl CustomPreviewStyle for Diff {
    fn to_list_item(&self, s: String) -> ListItem {
        if s.starts_with('+') {
            ListItem::new(Span::styled(s, Style::default().fg(Color::Green)))
        } else if s.starts_with('-') {
            ListItem::new(Span::styled(s, Style::default().fg(Color::Red)))
        } else {
            ListItem::new(Span::from(s))
        }
    }
}
