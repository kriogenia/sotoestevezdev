mod theme;

use crate::index::closeTty;

const HELP: &str = include_str!("../static/help.html");
const ABOUT: &str = include_str!("../static/about.html");

#[derive(Default)]
pub struct Shell {
    _history: Vec<String>,
}

// TODO: macro output

impl Shell {
    pub fn interpret(&mut self, line: String) -> Vec<String> {
        let mut args = line.trim().split_ascii_whitespace();

        match args.next() {
            None => Vec::new(),
            Some("about") => ABOUT.lines().map(|s| s.to_owned()).collect(),
            Some("help") => HELP.lines().map(|s| s.to_owned()).collect(),
            Some("exit") => {
                closeTty();
                vec!["Closing session...".to_string()]
            }
            Some("theme") => theme::run(args.next()),
            _ => vec!["Unknown command".to_string()],
        }
    }
}
