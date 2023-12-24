use crate::{processor, Token};

use super::processor::LineProcessor;

pub struct Lexer {
    processors: Vec<Box<dyn LineProcessor>>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            processors: vec![
                Box::new(processor::Metadata),
                Box::new(processor::MetadataPair::new()),
            ],
        }
    }

    pub fn process(&self, input: String) -> Vec<Token> {
        input.lines().map(|l| self.tokenize(l)).collect()
    }

    pub fn tokenize(&self, line: &str) -> Token {
        self.processors
            .iter()
            .skip_while(|&p| (*p).process_line(line).is_none())
            .next()
            .unwrap()
            .process_line(line)
            .unwrap_or(Token::Empty)
    }
}
