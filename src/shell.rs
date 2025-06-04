#[derive(Default)]
pub struct Shell {
    _history: Vec<String>,
}

impl Shell {
    pub fn interpret(&mut self, command: String) -> Vec<String> {
        let command = command.trim();
        match command {
            "" => Vec::new(),
            "help" => vec!["Help".to_string(), "Please".to_string()],
            _ => vec!["Unknown command".to_string()],
        }
    }
}
