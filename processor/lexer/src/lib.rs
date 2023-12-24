use crate::processor::LineProcessor;

mod lexer;
pub(crate) mod processor;

#[derive(Debug, PartialEq)]
pub enum Token {
    Metadata,
    MetadataPair(String, String),
    Empty,
    Markup(String),
    Header(u8, String),
}

pub struct Lexer {
    processors: Vec<Box<dyn LineProcessor>>,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            processors: vec![
                Box::new(processor::Empty),
                Box::new(processor::Metadata),
                Box::new(processor::MetadataPair::new()),
                Box::new(processor::Header::new()),
                Box::new(processor::Markup)
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
            .expect(&*format!("missing a processor capable of analyzing {line}"))
            .process_line(line)
            .unwrap_or(Token::Empty)
    }
}
#[cfg(test)]
mod tests {
    use std::any::Any;
    use super::*;

    const INPUT: &str = r#"---
+container: section
+classes: section
+legend: About
+command: about
---

# Title

![hey](hey) Me

## Subtitle

description
same paragraph
"#;

    #[test]
    fn it_works() {
        let result = Lexer::new().process(INPUT.to_owned());
        assert_eq!(2, result.iter().filter(|&token| *token == Token::Metadata).count());
        assert_eq!(4, result.iter().filter(|&token| matches!(*token, Token::MetadataPair(_, _))).count());
        assert_eq!(2, result.iter().filter(|&token| matches!(*token, Token::Header(_, _))).count());
        assert_eq!(3, result.iter().filter(|&token| matches!(*token, Token::Markup(_))).count());
        assert_eq!(4, result.iter().filter(|&token| *token == Token::Empty).count());
    }
}
