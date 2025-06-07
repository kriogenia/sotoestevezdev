mod theme;

const GREETING: &str = include_str!("../static/greeting.html");
const HELP: &str = include_str!("../static/help.html");
const ABOUT: &str = include_str!("../static/about.html");

#[derive(Default)]
pub struct Shell {
    _history: Vec<String>,
}

macro_rules! from_static {
    ($FILE:ident) => {
        $FILE.lines().map(|s| s.to_owned()).collect()
    };
}

impl Shell {
    pub fn interpret(&mut self, line: String) -> Vec<String> {
        let mut args = line.trim().split_ascii_whitespace();

        match args.next() {
            None => Vec::new(),
            Some("about") => from_static!(ABOUT),
            Some("exit") => exit::run(),
            Some("greeting") => from_static!(GREETING),
            Some("help") => from_static!(HELP),
            Some("theme") => theme::run(args.next()),
            _ => vec!["Unknown command".to_string()],
        }
    }

    pub fn greet(&self) -> Vec<String> {
        from_static!(GREETING)
    }
}

mod exit {

    use crate::index::closeTty;

    pub(super) fn run() -> Vec<String> {
        closeTty();
        vec!["Closing session...".to_string()]
    }
}
