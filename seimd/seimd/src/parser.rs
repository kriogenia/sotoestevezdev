use crate::html::SeimdHtmlTransformer;
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
        let (metadata, content): (Vec<Line>, Vec<Line>) = self.line_processor.process(input).into_iter()
            .filter(|l| !matches!(l, Line::Empty | Line::Metadata))
            .partition(|l|  matches!(l, Line::MetadataPair(_, _)));

        dbg!(&metadata);
        dbg!(&content);

        let metadata = metadata.into_iter().filter_map(|line| match line {
            Line::MetadataPair(key, value) => Some((key, value)),
            _ => None,
        }).collect();

        // not metadata -> map (<h1>transformed_header</h1>, <ul><li>transformed_list</li></ul>, <p>markup</p>) ->
        // -> join

        Parsed {
            metadata,
            html: String::new(),
        }
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
same paragraph wit `code

- [first item](https://url)
- second item
+ third item
with two lines
* fourth item
"#;

    #[test]
    fn parse() {
        let parser = SeimdParser::new();
        let result = parser.parse(INPUT.to_string());
        dbg!(&result);
        assert_eq!(2, result.metadata.len());
        assert_eq!("aside", result.metadata["container"]);
        assert_eq!("collapsible, nav", result.metadata["classes"]);
    }
}
