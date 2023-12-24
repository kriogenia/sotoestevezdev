use crate::{processor, Token};

use super::processor::LineProcessor;

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
    fn it_works() {
        let result = Lexer::new().process(INPUT.to_owned());
        for r in result {
            println!("{r:?}");
        }
    }
}
