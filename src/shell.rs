use std::str::FromStr;

use commands::Command;
use strum::IntoEnumIterator;

mod commands;
mod help;
mod theme;

const ABOUT: &str = include_str!("../static/about.html");
const CONTACT: &str = include_str!("../static/contact.html");
const GREETING: &str = include_str!("../static/greeting.html");
const LS: &str = ".rw-r--r--  42T sotoestevez  1 jan 13:37 .meaning-of-life.md";

#[derive(Default)]
pub struct Shell {
    history: Vec<String>,
    pointer: usize,
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
    ($content:tt) => {
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
        self.history.push(line.clone());
        self.pointer = 0;
        let mut args = line.trim().split_ascii_whitespace();

        // TODO: github, projects, projects <p>, techstack
        use Command::*;
        match args
            .next()
            .map(|c| Command::from_str(c).unwrap_or(Unknown))
            .unwrap_or(Empty)
        {
            Empty => Vec::new(),
            About => from_static!(ABOUT),
            Cat => cat::run(args.next()),
            Cd => to_vec!("cd: The directory{} does not exist", args),
            Clear => clear::run(),
            Cp => to_vec!("cp: cannot copy{}: Permission denied", args),
            Contact => from_static!(CONTACT),
            Echo => to_vec!((args.next().unwrap_or_default())),
            Theme => theme::run(args.next()),
            Editor => to_vec!("Oh, a dev of culture. I see"),
            Exit => exit::run(),
            Greeting => from_static!(GREETING),
            Grep => grep::run(args.next()),
            Help => help::run(),
            History => self.history.clone().into_iter().rev().skip(1).collect(),
            Hostname => to_vec!("sotoestevez"),
            Ls => to_vec!(LS),
            MissingEditor => to_vec!("Did you mean vim?"),
            MkDir => to_vec!("mkdir: cannot create directory{}: Permission denied", args),
            Mv => to_vec!("mv: cannot move{}: Permission denied", args),
            Pwd => to_vec!("/home/dev"),
            Rm => to_vec!("rm: cannot remove{}: Operation not permitted", args),
            RmDir => to_vec!("rmdir: failed to remove{}: Not a directory", args),
            Sudo => to_vec!("MUAHAHA YOU HAVE NO POWER HERE"),
            _ => to_vec!("Unknown command"),
        }
    }

    pub fn greet(&self) -> Vec<String> {
        from_static!(GREETING)
    }

    pub fn prev(&mut self) -> Option<&String> {
        if self.pointer >= self.history.len() {
            None
        } else {
            let prev = self.history.get(self.history.len() - 1 - self.pointer);
            self.pointer += 1;
            prev
        }
    }

    pub fn next(&mut self) -> Option<&String> {
        if self.pointer == 0 {
            None
        } else {
            let next = self.history.get(self.history.len() + 1 - self.pointer);
            self.pointer -= 1;
            next
        }
    }

    pub fn autocomplete_options(&self, input: &str) -> Vec<String> {
        Command::iter()
            .map(|c| c.to_string().to_ascii_lowercase())
            .filter(|c| !c.is_empty())
            .filter(|c| c.starts_with(input))
            .collect()
    }
}

mod clear {
    pub(super) fn run() -> Vec<String> {
        crate::index::clear();
        Default::default()
    }
}

mod cat {
    pub(super) fn run(file: Option<&str>) -> Vec<String> {
        let msg = match file {
            Some(".meaning-of-life.md") => "NO MIND IS READY TO DISCERN THE CONTENT OF THIS FILE",
            Some(f) => &format!("cat {}: No such file or directory", f),
            None => "dog",
        };
        to_vec!(msg)
    }
}

mod exit {
    pub(super) fn run() -> Vec<String> {
        crate::index::closeTty();
        to_vec!("Closing session...")
    }
}

mod grep {
    pub(super) fn run(search: Option<&str>) -> Vec<String> {
        vec![
            ".meaning-of-life.md:".to_string(),
            format!(" <em>42</em>: {}", search.unwrap_or_default()),
        ]
    }
}
