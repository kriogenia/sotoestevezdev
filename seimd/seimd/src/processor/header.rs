use regex::Regex;
use crate::processor::LineProcessor;
use crate::Token;

pub struct Header {
    re: Regex
}

impl Header {
    pub fn new() -> Self {
        Self {
            re: Regex::new(r"^(#{1,6}) +(.*)$").unwrap()
        }
    }
}

impl LineProcessor for Header {
    fn process_line(&self, line: &str) -> Option<Token> {
        self.re.captures(line)
            .map(|caps| (caps[1].len(), caps[2].to_string()))
            .map(|(number, title)| Token::Header(number as u8, title))
    }
}

#[cfg(test)]
mod tests {
    use crate::processor::header::Header;
    use crate::processor::LineProcessor;
    use crate::Token;

    #[test]
    fn valid_header() {
        let processor = Header::new();
        assert_eq!(Some(Token::Header(3, "h3".to_string())), processor.process_line("### h3"));
        assert_eq!(Some(Token::Header(1, "h1".to_string())), processor.process_line("#   h1"));
    }

    #[test]
    fn invalid_header() {
        let processor = Header::new();
        assert_eq!(None, processor.process_line("missing #"));
        assert_eq!(None, processor.process_line("#missing space"));
        assert_eq!(None, processor.process_line(" #wrong start"));
    }


}