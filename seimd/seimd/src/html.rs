mod transformation;

use crate::html::transformation::HtmlTransformation;

pub trait HtmlTransformer {
    fn transform(&self, element: &str) -> String;
}

pub struct SeimdHtmlTransformer {
    transformers: Vec<HtmlTransformation>,
}

impl Default for SeimdHtmlTransformer {
    fn default() -> Self {
        Self {
            transformers: HtmlTransformation::ordered_seq(),
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

#[cfg(test)]
mod tests {
    use crate::html::{HtmlTransformer, SeimdHtmlTransformer};

    const INPUT: &str =
        "**This _complex_ *line contains* `all` the ***supported*** transformations**. \
    Even ![alt](url), [the __links__](http://link) and ~~lists~~";

    #[test]
    fn generates_html() {
        let expected = "<strong>This <em>complex</em> <em>line contains</em> <code>all</code> the \
        <strong><em>supported</em></strong> transformations</strong>. Even <img href=\"url\" alt=\"alt\"/>, \
        <a href=\"http://link\">the <strong>links</strong></a> and <del>lists</del>";
        assert_eq!(expected, SeimdHtmlTransformer::default().transform(INPUT));
    }
}
