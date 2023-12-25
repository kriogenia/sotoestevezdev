use crate::html::HtmlTransformer;
use regex::{Captures, Regex};

pub struct Bold {
    expressions: Vec<Regex>,
}

impl Bold {
    pub fn new() -> Self {
        Self {
            expressions: vec![
                Regex::new(r"\*\*(.*)\*\*").unwrap(),
                Regex::new(r"__(.*)___").unwrap(),
            ],
        }
    }
}

impl HtmlTransformer for Bold {
    fn transform(&self, element: &str) -> String {
        self.expressions
            .iter()
            .fold(element.to_string(), |element, re| {
                super::sequential_replace(&element, re, |caps: &Captures| {
                    format!("<strong>{}</strong>", &caps[1])
                })
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::html::HtmlTransformer;

    #[test]
    fn asterisk_notation() {
        assert_transform("**Multiple** bold **segments**")
    }

    #[test]
    fn underscore_notation() {
        assert_transform("__Multiple__ bold __segments__")
    }

    #[test]
    fn mixed_notations() {
        assert_transform("__Multiple__ bold **segments**")
    }

    fn assert_transform(input: &str) {
        let bold = Bold::new();
        assert_eq!(
            "<strong>Multiple</strong> bold <strong>segments</strong>",
            bold.transform(input)
        );
    }
}
