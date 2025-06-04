const HELP: &str = include_str!("../static/help.html");
const ABOUT: &str = include_str!("../static/about.html");

#[derive(Default)]
pub struct Shell {
    _history: Vec<String>,
}

impl Shell {
    pub fn interpret(&mut self, command: String) -> Vec<String> {
        let command = command.trim();
        match command {
            "" => Vec::new(),
            "help" => HELP.lines().map(|s| s.to_owned()).collect(),
            "about" => ABOUT.lines().map(|s| s.to_owned()).collect(),
            _ => vec!["Unknown command".to_string()],
        }
    }
}
