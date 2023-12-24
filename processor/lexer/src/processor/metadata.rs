use regex::Regex;

use crate::Token;

use super::LineProcessor;

const METADATA_LINE: &str = "---";

pub struct Metadata;

impl LineProcessor for Metadata {
    fn process_line(&self, line: &str) -> Option<Token> {
        if line == METADATA_LINE {
            Some(Token::Metadata)
        } else {
            None
        }
    }
}

pub struct MetadataPair {
    re: Regex,
}

impl MetadataPair {
    pub fn new() -> Self {
        Self {
            re: Regex::new(r"^\+([a-z]+): *(.+)$").unwrap(),
        }
    }
}

impl LineProcessor for MetadataPair {
    fn process_line(&self, line: &str) -> Option<Token> {
        self.re
            .captures(line)
            .map(|c| Token::MetadataPair(c[1].to_string(), c[2].to_string()))
    }
}

#[cfg(test)]
mod tests {
    use crate::processor::{LineProcessor, Metadata, MetadataPair};
    use crate::Token;

    #[test]
    fn metadata_line() {
        assert_eq!(Some(Token::Metadata), Metadata.process_line("---"));
        assert_eq!(None, Metadata.process_line("class: bold, open"));
    }

    #[test]
    fn valid_metadata_pair() {
        let expected = Some(Token::MetadataPair(
            "class".to_string(),
            "bold, open".to_string(),
        ));
        assert_eq!(
            expected,
            MetadataPair::new().process_line("+class: bold, open")
        );
        assert_eq!(
            expected,
            MetadataPair::new().process_line("+class:bold, open")
        );
    }

    #[test]
    fn invalid_metadata_pair() {
        assert_eq!(None, MetadataPair::new().process_line("missing: plus sign"));
        assert_eq!(None, MetadataPair::new().process_line("+missing colons"));
        assert_eq!(None, MetadataPair::new().process_line("+missing_value:"));
        assert_eq!(None, MetadataPair::new().process_line("+: mising key"));
    }
}
