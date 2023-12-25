mod bold;

use crate::html::bold::Bold;
use regex::{Captures, Regex};

pub trait HtmlTransformer {
    fn transform(&self, element: &str) -> String;
}

pub struct SeimdHtmlTransformer {
    transformers: Vec<Box<dyn HtmlTransformer>>,
}

impl Default for SeimdHtmlTransformer {
    fn default() -> Self {
        Self {
            transformers: vec![Box::new(Bold::new())],
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
    **This _complex_ *line contains* `all` the [README.md](supported) transformations**.
    Even ![images](images) __and__ ~lists~"#;

    #[test]
    fn it_works() {
        let expected = r#"
    <strong>This _complex_ *line contains* `all` the [README.md](supported) transformations</strong>.
    Even ![images](images) <strong>and</strong> ~lists~"#;
        assert_eq!(expected, SeimdHtmlTransformer::default().transform(INPUT));
    }
}
