use seimd::Parsed;

pub enum HtmlDecorator {
    ParentWrapper,
    Legend,
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
            Self::Legend => parsed
                .metadata
                .get("legend")
                .map(|legend| format!("<legend>{legend}</legend>{html}")),
        }
        .unwrap_or(html)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use seimd::Parsed;
    use crate::decoration::HtmlDecorator;

    #[test]
    fn parent_wrapper() {
        let mut metadata = HashMap::new();
        metadata.insert("parent_tag".to_string(), "div".to_string());
        let mut parsed = Parsed { metadata, html: Default::default(), };

        let result = HtmlDecorator::ParentWrapper.decorate(&parsed, "html".to_string());
        assert_eq!(result, "<div>html</div>");

        parsed.metadata.insert("parent_class".to_string(), "active, valid".to_string());
        let result = HtmlDecorator::ParentWrapper.decorate(&parsed, "html".to_string());
        assert_eq!(result, "<div class=\"active, valid\">html</div>");
    }

    #[test]
    fn legend() {
        let mut metadata = HashMap::new();
        metadata.insert("legend".to_string(), "Legend".to_string());
        let parsed = Parsed { metadata, html: Default::default(), };

        let result = HtmlDecorator::Legend.decorate(&parsed, "html".to_string());
        assert_eq!(result, "<legend>Legend</legend>html");
    }

}
