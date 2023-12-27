use regex::{Captures, Regex};
use std::slice::Iter;
use std::sync::OnceLock;

macro_rules! surround_with {
    ($tag:tt) => {
        |caps: &Captures| format!("<{}>{}</{}>", $tag, &caps[1], $tag)
    };
}

pub enum HtmlTransformation {
    Bold,
    Italic,
    Link,
}

impl HtmlTransformation {
    pub fn transform(&self, element: &str) -> String {
        self.regex().fold(element.to_string(), |element, re| {
            self.sequential_replace(&element, re)
        })
    }

    fn regex(&self) -> Iter<Regex> {
        match self {
            Self::Bold => BOLD_RE.get_or_init(|| {
                vec![
                    Regex::new(r"[*]{2}(.*)[*]{2}").unwrap(),
                    Regex::new(r"_{2}(.*)_{2}").unwrap(),
                ]
            }),
            Self::Italic => ITALIC_RE.get_or_init(|| {
                vec![
                    Regex::new(r"[*](.*)[*]").unwrap(),
                    Regex::new(r"_(.*)_").unwrap(),
                ]
            }),
            Self::Link => LINK_RE.get_or_init(|| {
                vec![
                    Regex::new(r"\[(.+)]\((.+)\)").unwrap(),
                    Regex::new(r"<(https?://.+)>").unwrap(),
                ]
            })
        }
        .iter()
    }

    fn replacer(&self) -> fn(&Captures) -> String {
        match self {
            Self::Bold => surround_with!("strong"),
            Self::Italic => surround_with!("em"),
            Self::Link => |caps| {
                let href = caps.get(2).map(|m| m.as_str()).unwrap_or(&caps[1]);
                format!("<a href=\"{href}\">{}</a>", &caps[1])
            }
        }
    }

    fn sequential_replace(&self, element: &str, re: &Regex) -> String {
        let mut remaining = &element[..];
        let mut acc = "".to_string();
        while let Some(offset) = re.shortest_match(remaining) {
            acc += &*re.replace(&remaining[..offset], self.replacer());
            remaining = &remaining[offset..];
        }
        acc += remaining;
        acc
    }
}

static BOLD_RE: OnceLock<Vec<Regex>> = OnceLock::new();
static ITALIC_RE: OnceLock<Vec<Regex>> = OnceLock::new();
static LINK_RE: OnceLock<Vec<Regex>> = OnceLock::new();

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bold() {
        assert_surround(HtmlTransformation::Bold, "strong", "**");
        assert_surround(HtmlTransformation::Bold, "strong", "__");
        assert_surrounds(HtmlTransformation::Bold, "strong", "**", "__");
    }

    #[test]
    fn italic() {
        assert_surround(HtmlTransformation::Italic, "em", "*");
        assert_surround(HtmlTransformation::Italic, "em", "_");
        assert_surrounds(HtmlTransformation::Italic, "em", "*", "_");
    }

    #[test]
    fn link() {
        assert_eq!(
            "Parses <a href=\"https://url\">text</a> links",
            HtmlTransformation::Link.transform("Parses [text](https://url) links")
        );
        assert_eq!(
            "Parses secure <a href=\"https://url\">https://url</a>",
            HtmlTransformation::Link.transform("Parses secure <https://url>")
        );
        assert_eq!(
            "Parses just http <a href=\"http://url\">http://url</a>",
            HtmlTransformation::Link.transform("Parses just http <http://url>")
        );
        assert_eq!(
            "Parses just <invalid://url>",
            HtmlTransformation::Link.transform("Parses just <invalid://url>")
        );
    }

    fn assert_surround(transformer: HtmlTransformation, html: &str, md: &str) {
        assert_surrounds(transformer, html, md, md);
    }

    fn assert_surrounds(transformer: HtmlTransformation, html: &str, first: &str, second: &str) {
        assert_eq!(
            format!("Multiple <{html}>marked</{html}> segments <{html}>in one</{html}> line"),
            transformer.transform(&format!(
                "Multiple {first}marked{first} segments {second}in one{second} line"
            ))
        )
    }
}
