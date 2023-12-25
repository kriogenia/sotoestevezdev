use itertools::Itertools;

mod empty;
mod header;
mod markup;
mod metadata;

#[derive(Debug, PartialEq)]
pub enum Line {
    Metadata,
    MetadataPair(String, String),
    Empty,
    Markup(String),
    Header(u8, String),
}

impl Line {
    fn key(&self) -> String {
        match self {
            Line::Metadata => "metadata",
            Line::MetadataPair(_, _) => "metadata_pair",
            Line::Empty => "empty",
            Line::Markup(_) => "markup",
            Line::Header(_, _) => "header",
        }
        .to_string()
    }

    pub fn join(&mut self, next: Line) -> Line {
        match (self, next) {
            (Line::Markup(prev), Line::Markup(next)) => Line::Markup(format!("{prev} {next}")),
            (Line::Empty, Line::Empty) => Line::Empty,
            (_, _) => unreachable!("no other type should be attempted to be merged"),
        }
    }
}

pub trait LineProcessor {
    fn process_line(&self, line: &str) -> Option<Line>;
}

pub struct SeimdLineProcessor {
    processors: Vec<Box<dyn LineProcessor>>,
}

impl SeimdLineProcessor {

    pub fn process(&self, input: String) -> Vec<Line> {
        input
            .lines()
            .map(|line| self.tokenize(line))
            .group_by(|line| line.key())
            .into_iter()
            .map(|(key, value)| match key.as_str() {
                "empty" | "markup" => value
                    .into_iter()
                    .reduce(|mut acc, line| acc.join(line))
                    .into_iter()
                    .collect::<Vec<_>>(),
                _ => value.into_iter().collect::<Vec<_>>(),
            })
            .flatten()
            .collect()
        //input.lines().map(|line| self.tokenize(line)).collect()
    }

    pub fn tokenize(&self, line: &str) -> Line {
        self.processors
            .iter()
            .skip_while(|&p| (*p).process_line(line).is_none())
            .next()
            .expect(format!("a line capable of analyzing {line}").as_str())
            .process_line(line)
            .unwrap()
    }
}

impl Default for SeimdLineProcessor {
    fn default() -> Self {
        Self {
            processors: vec![
                Box::new(empty::Empty),
                Box::new(metadata::Metadata),
                Box::new(metadata::MetadataPair::new()),
                Box::new(header::Header::new()),
                Box::new(markup::Markup),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"---
+container: section
+classes: section
+legend: About
+command: about
---


# Title

![hey](hey) Me

## Subtitle

description
same paragraph
"#;

    #[test]
    fn tokenize_lines() {
        let result = SeimdLineProcessor::default().process(INPUT.to_owned());
        assert_eq!(
            2,
            result
                .iter()
                .filter(|&token| *token == Line::Metadata)
                .count()
        );
        assert_eq!(
            4,
            result
                .iter()
                .filter(|&token| matches!(*token, Line::MetadataPair(_, _)))
                .count()
        );
        assert_eq!(
            2,
            result
                .iter()
                .filter(|&token| matches!(*token, Line::Header(_, _)))
                .count()
        );
        assert_eq!(
            2,
            result
                .iter()
                .filter(|&token| matches!(*token, Line::Markup(_)))
                .count()
        );
        assert_eq!(
            4,
            result.iter().filter(|&token| *token == Line::Empty).count()
        );
    }
}
