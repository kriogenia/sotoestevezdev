use regex::Regex;

use crate::Token;

use super::LineProcessor;

pub struct MetadataPair {
    re: Regex,
}

impl MetadataPair {
    pub fn new() -> Self {
        Self {
            re: Regex::new(r"^\+[a-z]+: [a-z]+$").unwrap(),
        }
    }
}

impl LineProcessor for MetadataPair {
    fn process_line(&self, line: &str) -> Option<crate::Token> {
        if self.re.is_match(line) {
            return Some(Token::MetadataPair("a".to_string(), "b".to_string()));
        }
        None
    }
}
