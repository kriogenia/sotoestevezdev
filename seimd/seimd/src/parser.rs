use crate::html::{HtmlTransformer, SeimdHtmlTransformer};
use crate::line::{Line, SeimdLineProcessor};
use crate::Parsed;

#[derive(Default)]
pub struct SeimdParser {
    line_processor: SeimdLineProcessor,
    html_parser: SeimdHtmlTransformer,
}

impl SeimdParser {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn parse(&self, input: String) -> Parsed {
        let (metadata, content): (Vec<Line>, Vec<Line>) = self
            .line_processor
            .process(input)
            .into_iter()
            .filter(|l| !matches!(l, Line::Empty | Line::Metadata))
            .partition(|l| matches!(l, Line::MetadataPair(_, _)));

        let metadata = metadata
            .into_iter()
            .filter_map(|line| match line {
                Line::MetadataPair(key, value) => Some((key, value)),
                _ => None,
            })
            .collect();

        let html: String = content
            .into_iter()
            .map(|l| match l {
                Line::Header(n, content) => {
                    format!("<h{n}>{}</h{n}>", self.html_parser.transform(&content))
                }
                Line::Markup(content) => format!("<p>{}</p>", self.html_parser.transform(&content)),
                Line::UnorderedList(items) => {
                    let items = items
                        .iter()
                        .map(|i| format!("<li>{}</li>", self.html_parser.transform(i)))
                        .reduce(|acc, li| acc + &li)
                        .unwrap_or_default();
                    format!("<ul>{items}</ul>")
                }
                _ => "".to_string(),
            })
            .reduce(|acc, next| acc + &next)
            .unwrap_or_default();

        Parsed { metadata, html }
    }
}

#[cfg(test)]
mod tests {
    use crate::parser::SeimdParser;

    const INPUT: &str = r#"---
+container: aside
+classes: collapsible, nav
---


# Title

![hey](hey) Me

## _Italic Subtitle_

description, **bold**
same paragraph with `code`

- [first item](https://url)
- ***second item***
+ third item
with two lines
* ~~fourth item~~
"#;

    #[test]
    fn parse() {
        let parser = SeimdParser::new();
        let result = parser.parse(INPUT.to_string());

        assert_eq!(2, result.metadata.len());
        assert_eq!("aside", result.metadata["container"]);
        assert_eq!("collapsible, nav", result.metadata["classes"]);

        let expected_html = "<h1>Title</h1>\
        <p><img href=\"hey\" alt=\"hey\"/> Me</p>\
        <h2><em>Italic Subtitle</em></h2>\
        <p>description, <strong>bold</strong> same paragraph with <code>code</code></p>\
        <ul><li><a href=\"https://url\">first item</a></li>\
        <li><strong><em>second item</em></strong></li>\
        <li>third item with two lines</li>\
        <li><del>fourth item</del></li></ul>";
        assert_eq!(expected_html, result.html);
    }
}
