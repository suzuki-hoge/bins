use bins::libs::app::input_app::InputApp;
use bins::libs::app::paged_select_app::PagedSelectApp;
use bins::libs::common::matched_string::MatchedString;

#[derive(Debug)]
pub struct App {
    pub input_app: InputApp,
    pub paged_select_app: PagedSelectApp<String, MatchedString>,
    fixed_items: Vec<String>,
}

impl App {
    pub fn init(items: Vec<String>) -> Self {
        let mut s =
            Self { input_app: InputApp::init(), paged_select_app: PagedSelectApp::init(items), fixed_items: vec![] };
        s.paged_select_app.re_match(|item| MatchedString::matched_only("", &item));
        s.paged_select_app.re_page();
        s
    }

    pub fn refresh(&mut self) {
        self.paged_select_app.re_match(|item| MatchedString::matched_only(&self.input_app.input, &item));
        self.paged_select_app.re_render();
    }

    pub fn fix(&mut self) {
        let item = self.paged_select_app.pop_item();
        self.fixed_items.push(item);
    }

    pub fn finish(self) -> Vec<String> {
        self.fixed_items
    }
}

#[cfg(test)]
mod tests {
    use crate::app::App;

    #[test]
    fn fix() {
        let mut app =
            App::init(vec!["youtube", "github", "instagram", "twitter"].iter().map(|s| s.to_string()).collect());
        app.set_per_page(20);
        println!("{:?}", (&app));

        // input char

        app.input_app.insert('e');
        app.refresh();

        assert!(app.matched_lines[0].is_some());
        assert!(app.matched_lines[1].is_none());
        assert!(app.matched_lines[2].is_none());
        assert!(app.matched_lines[3].is_some());

        // fix

        app.down();
        app.fix();

        assert!(app.matched_lines[0].is_some());
        assert!(app.matched_lines[1].is_none());
        assert!(app.matched_lines[2].is_none());
        assert_eq!(app.fixed_lines, vec!["twitter"]);

        // delete

        app.input_app.remove();
        app.refresh();

        // fix

        app.fix();

        assert!(app.matched_lines[0].is_some());
        assert!(app.matched_lines[1].is_some());
        assert_eq!(app.fixed_lines, vec!["twitter", "youtube"]);
    }

    #[test]
    fn page() {
        let mut app = App::init(
            vec!["+a", "+b", "-c", "+d", "+e", "-f", "+g", "+h", "-i", "+j"].iter().map(|s| s.to_string()).collect(),
        );
        app.set_per_page(20);
        app.input_app.insert('+');
        app.refresh();
        // &app.matched_lines.iter().for_each(|ml| println!("{:?}", ml));
        println!("{:?}", (&app.origin_lines2));
        println!("{:?}", (&app.matched_lines2));
        println!("{:?}", (&app.matched_line_numbers));
    }
}
