use super::LineProcessor;
use crate::Token;

pub struct Metadata;

impl LineProcessor for Metadata {
    fn process_line(&self, line: &str) -> Option<Token> {
        if line == "---" {
            Some(Token::Metadata)
        } else {
            None
        }
    }
}
