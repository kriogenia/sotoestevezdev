mod bold;
mod image;

use crate::html::bold::Bold;
use regex::{Captures, Regex};
use crate::html::image::Image;

pub trait HtmlTransformer {
    fn transform(&self, element: &str) -> String;
}

pub struct SeimdHtmlTransformer {
    transformers: Vec<Box<dyn HtmlTransformer>>,
}

impl Default for SeimdHtmlTransformer {
    fn default() -> Self {
        Self {
            transformers: vec![
                Box::new(Image::default()),
                Box::new(Bold::default())
            ],
        }
    }
}

impl HtmlTransformer for SeimdHtmlTransformer {
    fn transform(&self, element: &str) -> String {
        self.transformers
            .iter()
            .fold(element.to_string(), |element, transformer| {
                transformer.transform(&element)
            })
    }
}

fn sequential_replace(element: &str, re: &Regex, replacer: fn(&Captures) -> String) -> String {
    let mut remaining = &element[..];
    let mut acc = "".to_string();
    while let Some(offset) = re.shortest_match(remaining) {
        acc += &*re.replace(&remaining[..offset], replacer);
        remaining = &remaining[offset..];
    }
    acc += remaining;
    acc
}

#[cfg(test)]
mod tests {
    use crate::html::{HtmlTransformer, SeimdHtmlTransformer};

    const INPUT: &str = r#"
    **This _complex_ *line contains* `all` the ***supported*** transformations**.
    Even ![alt](url), [the __links__](http://link) __and__ ~lists~"#;

    #[test]
    fn generates_html() {
        let expected = r#"
    <strong>This _complex_ *line contains* `all` the ***supported** transformations</strong>.
    Even <img href=\"url\" alt=\"alt\"/>, [the <strong>links</strong>](http://link) <strong>and</strong> ~lists~"#;
        assert_eq!(expected, SeimdHtmlTransformer::default().transform(INPUT));
    }
}
