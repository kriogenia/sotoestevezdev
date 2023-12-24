mod lexer;
pub(crate) mod processor;

use crate::lexer::Lexer;

#[derive(Debug)]
pub enum Token {
    Metadata,
    MetadataPair(String, String),
    Empty,
    Markup(String),
    Header(u8, String),
}
pub fn process(input: String) -> Vec<Token> {
    let lexer = Lexer::new();
    lexer.process(input)
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

![hey](hey) Me

## Title

description
same paragraph
"#;

    const A: &str = r#"---
+container: section"#;

    #[test]
    fn not_works() {
        let result = process(INPUT.to_owned());
    }

    #[test]
    fn it_does() {
        let result = process(A.to_owned());
        for r in result {
            println!("{r:?}");
        }
    }
}
