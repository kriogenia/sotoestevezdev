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
                    .get("parent_classes")
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
