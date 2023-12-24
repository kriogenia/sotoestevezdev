use crate::processor::LineProcessor;
use crate::Token;

pub struct Markup;

impl LineProcessor for Markup {
    fn process_line(&self, line: &str) -> Option<Token> {
        Some(Token::Markup(line.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::processor::{LineProcessor, Markup};
    use crate::Token;

    #[test]
    fn wraps_line() {
        let line = "no matter what **this** _should work_";
        assert_eq!(
            Some(Token::Markup(line.to_string())),
            Markup.process_line(&line)
        );
    }

}