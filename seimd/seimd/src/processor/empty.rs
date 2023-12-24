use crate::processor::LineProcessor;
use crate::Token;

pub struct Empty;

impl LineProcessor for Empty {
    fn process_line(&self, line: &str) -> Option<Token> {
        if line.trim().is_empty() {
            Some(Token::Empty)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::processor::LineProcessor;
    use crate::Token;

    #[test]
    fn empty_line() {
        assert_eq!(Some(Token::Empty), Empty.process_line(""));
        assert_eq!(Some(Token::Empty), Empty.process_line("     "));
        assert_eq!(Some(Token::Empty), Empty.process_line("\t"));
    }

    #[test]
    fn non_empty_line() {
        assert_eq!(None, Empty.process_line("class: bold, open"));
    }
}