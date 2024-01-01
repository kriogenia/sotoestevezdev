use seimd::Parsed;

#[derive(PartialEq)]
pub enum HtmlDecorator {
    ParentWrapper,
    Summary,
}

impl HtmlDecorator {
    pub fn decorate(&self, parsed: &Parsed, html: String) -> String {
        match self {
            Self::ParentWrapper => parsed.metadata.get("parent_tag").map(|tag| {
                let classes = parsed
                    .metadata
                    .get("parent_class")
                    .map(|classes| format!(" class=\"{classes}\""))
                    .unwrap_or_default();
                format!("<{tag}{classes}>{html}</{tag}>")
            }),
            Self::Summary => parsed
                .metadata
                .get("summary")
                .map(|summary| format!("<summary>{summary}</summary>{html}")),
        }
        .unwrap_or(html)
    }
}

#[derive(Default)]
pub struct DecoratorBuilder {
    pub decorators: Vec<HtmlDecorator>,
}

impl DecoratorBuilder {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn summary(mut self, summary: bool) -> Self {
        if summary && !self.decorators.contains(&HtmlDecorator::Summary) {
            self.decorators.push(HtmlDecorator::Summary);
        }
        self
    }

    pub fn parent(mut self, parent: bool) -> Self {
        if parent && !self.decorators.contains(&HtmlDecorator::ParentWrapper) {
            self.decorators.push(HtmlDecorator::ParentWrapper);
        }
        self
    }

    pub fn build(self) -> Vec<HtmlDecorator> {
        self.decorators
    }
}

#[cfg(test)]
mod tests {
    use crate::decoration::HtmlDecorator;
    use seimd::Parsed;
    use std::collections::HashMap;

    #[test]
    fn parent_wrapper() {
        let mut metadata = HashMap::new();
        metadata.insert("parent_tag".to_string(), "div".to_string());
        let mut parsed = Parsed {
            metadata,
            html: Default::default(),
        };

        let result = HtmlDecorator::ParentWrapper.decorate(&parsed, "html".to_string());
        assert_eq!(result, "<div>html</div>");

        parsed
            .metadata
            .insert("parent_class".to_string(), "active, valid".to_string());
        let result = HtmlDecorator::ParentWrapper.decorate(&parsed, "html".to_string());
        assert_eq!(result, "<div class=\"active, valid\">html</div>");
    }

    #[test]
    fn summary() {
        let mut metadata = HashMap::new();
        metadata.insert("summary".to_string(), "Summary".to_string());
        let parsed = Parsed {
            metadata,
            html: Default::default(),
        };

        let result = HtmlDecorator::Summary.decorate(&parsed, "html".to_string());
        assert_eq!(result, "<summary>Summary</summary>html");
    }
}
