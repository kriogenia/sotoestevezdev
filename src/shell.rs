mod theme;

const ABOUT: &str = include_str!("../static/about.html");
const GREETING: &str = include_str!("../static/greeting.html");
const HELP: &str = include_str!("../static/help.html");
const LS: &str = ".rw-r--r--  42T sotoestevez  1 jan 13:37 .meaning-of-life.md";

#[derive(Default)]
pub struct Shell {
    _history: Vec<String>,
}

macro_rules! from_static {
    ($FILE:ident) => {
        $FILE.lines().map(|s| s.to_owned()).collect()
    };
}

macro_rules! to_vec {
    ($content:literal) => {
        vec![$content.to_string()]
    };
    ($content:ident) => {
        vec![$content.to_string()]
    };
    ($tpl:literal, $args:tt) => {{
        let dir = $args
            .next()
            .map(|d| format!(" '{}'", d))
            .unwrap_or_default();
        let msg = format!($tpl, dir);
        to_vec![msg]
    }};
}

impl Shell {
    pub fn interpret(&mut self, line: String) -> Vec<String> {
        let mut args = line.trim().split_ascii_whitespace();

        // TODO: contact, github, projects, projects, history, clear, techstack, easter eggs <p>
        // cat meaning of life: you cant, other file not exists
        // grep rg always find the
        // nano, do you mean vim?
        match args.next() {
            None => Vec::new(),
            Some("about") => from_static!(ABOUT),
            Some("cd") => to_vec!("cd: The directory{} does not exist", args),
            Some("cp") => to_vec!("cp: cannot copy{}: Permission denied", args),
            Some("exit") => exit::run(),
            Some("greeting") => from_static!(GREETING),
            Some("help") => from_static!(HELP),
            Some("hostname") => to_vec!("sotoestevez"),
            Some("ls" | "eza") => to_vec!(LS),
            Some("mkdir") => to_vec!("mkdir: cannot create directory{}: Permission denied", args),
            Some("mv") => to_vec!("mv: cannot move{}: Permission denied", args),
            Some("pwd") => to_vec!("/home/dev"),
            Some("rm") => to_vec!("rm: cannot remove {}: Operation not permitted", args),
            Some("rmdir") => to_vec!("rmdir: failed to remove {}: Not a directory", args),
            Some("theme") => theme::run(args.next()),
            _ => to_vec!("Unknown command"),
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
        to_vec!("Closing session...")
    }
}
