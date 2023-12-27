use std::iter;

mod empty;
mod header;
mod markup;
mod metadata;
mod unordered_list;

#[derive(Debug, PartialEq)]
pub enum Line {
    Metadata,
    MetadataPair(String, String),
    Header(u8, String),
    UnorderedList(Vec<String>),
    Markup(String),
    Empty,
}

impl Line {
    fn key(&self) -> String {
        match self {
            Line::Metadata => "metadata",
            Line::MetadataPair(_, _) => "metadata_pair",
            Line::Header(_, _) => "header",
            Line::UnorderedList(_) => "unordered_list",
            Line::Markup(_) => "markup",
            Line::Empty => "empty",
        }
        .to_string()
    }

    pub fn join(&self, next: &Self) -> Option<Self> {
        match (self, next) {
            (Line::UnorderedList(prev), Line::UnorderedList(next)) => Some(Line::UnorderedList(
                prev.iter().chain(next.iter()).cloned().collect(),
            )),
            (Line::UnorderedList(list), Line::Markup(line)) => {
                let last = format!("{} {line}", list.last().unwrap());
                Some(Line::UnorderedList(
                    iter::once(last)
                        .chain(list.iter().cloned().rev().skip(1))
                        .rev()
                        .collect(),
                ))
            }
            (Line::Markup(prev), Line::Markup(next)) => {
                Some(Line::Markup(format!("{prev} {next}")))
            }
            (Line::Empty, Line::Empty) => Some(Line::Empty),
            (_, _) => None,
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
        let mut result = Vec::new();
        let mut lines = input.lines().map(|line| self.tokenize(line));

        let mut prev = lines.next().unwrap_or(Line::Empty);
        for line in lines {
            if let Some(merge) = prev.join(&line) {
                prev = merge;
            } else {
                result.push(prev);
                prev = line;
            }
        }
        result.push(prev);

        result
    }

    pub fn tokenize(&self, line: &str) -> Line {
        self.processors
            .iter()
            .find(|&p| (*p).process_line(line).is_some())
            .unwrap_or_else(|| panic!("a line capable of analyzing {line}"))
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
                Box::<unordered_list::UnorderedList>::default(),
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

- first item
- second item
+ third item
with two lines
* fourth item
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
            "first item;second item;third item with two lines;fourth item",
            result
                .iter()
                .filter_map(|token| if let Line::UnorderedList(vec) = token {
                    Some(vec.join(";"))
                } else {
                    None
                })
                .next()
                .unwrap()
                .as_str()
        );
        assert_eq!(
            2,
            result
                .iter()
                .filter(|&token| matches!(*token, Line::Markup(_)))
                .count()
        );
        assert_eq!(
            5,
            result.iter().filter(|&token| *token == Line::Empty).count()
        );
    }

    #[test]
    fn join() {
        assert_eq!((Line::Empty), Line::Empty.join(&Line::Empty).unwrap());
        assert_eq!(
            Line::Markup("a b".to_string()),
            Line::Markup("a".to_string())
                .join(&Line::Markup("b".to_string()))
                .unwrap()
        );
        assert_eq!(
            Line::UnorderedList(vec!["a".to_string(), "b".to_string()]),
            Line::UnorderedList(vec!["a".to_string()])
                .join(&Line::UnorderedList(vec!["b".to_string()]))
                .unwrap()
        );
        assert_eq!(
            Line::UnorderedList(vec!["a".to_string(), "b a".to_string()]),
            Line::UnorderedList(vec!["a".to_string(), "b".to_string()])
                .join(&Line::Markup("a".to_string()))
                .unwrap()
        )
    }
}
