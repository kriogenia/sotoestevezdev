use crate::html::HtmlTransformer;
use regex::{Captures, Regex};

pub struct Bold {
    expressions: Vec<Regex>,
}

impl Default for Bold {
    fn default() -> Self {
        Self {
            expressions: vec![
                Regex::new(r"[*]{2}(.*)[*]{2}").unwrap(),
                Regex::new(r"_{2}(.*)_{2}").unwrap(),
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
        assert_transform("Multiple **bold** segments **in one** line")
    }

    #[test]
    fn underscore_notation() {
        assert_transform("Multiple __bold__ segments __in one__ line")
    }

    #[test]
    fn mixed_notations() {
        assert_transform("Multiple __bold__ segments **in one** line")
    }

    #[test]
    fn ignores_italic_and_bolditalic() {
        assert_eq!(
            "*italic* and ***bold_italic***",
            Bold::default().transform("*italic* and ***bold_italic***")
        );
    }

    fn assert_transform(input: &str) {
        let bold = Bold::default();
        assert_eq!(
            "Multiple <strong>bold</strong> segments <strong>in one</strong> line",
            bold.transform(input)
        );
    }
}
