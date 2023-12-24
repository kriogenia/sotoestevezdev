mod empty;
mod metadata;
mod header;
mod markup;

use crate::Token;


pub trait LineProcessor {
    fn process_line(&self, line: &str) -> Option<Token>;
}

pub struct SeimdProcessor {
    processors: Vec<Box<dyn LineProcessor>>,
}

impl SeimdProcessor {
    pub fn new() -> Self {
        Self {
            processors: vec![
                Box::new(empty::Empty),
                Box::new(metadata::Metadata),
                Box::new(metadata::MetadataPair::new()),
                Box::new(header::Header::new()),
                Box::new(markup::Markup)
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
    fn tokenize_lines() {
        let result = SeimdProcessor::new().process(INPUT.to_owned());
        assert_eq!(2, result.iter().filter(|&token| *token == Token::Metadata).count());
        assert_eq!(4, result.iter().filter(|&token| matches!(*token, Token::MetadataPair(_, _))).count());
        assert_eq!(2, result.iter().filter(|&token| matches!(*token, Token::Header(_, _))).count());
        assert_eq!(3, result.iter().filter(|&token| matches!(*token, Token::Markup(_))).count());
        assert_eq!(4, result.iter().filter(|&token| *token == Token::Empty).count());
    }
}

