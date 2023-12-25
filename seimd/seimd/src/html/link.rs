use regex::{Captures, Regex};
use crate::html::HtmlTransformer;

pub struct Link {
    with_text: Regex,
    only_link: Regex,
}

impl Default for Link {
    fn default() -> Self {
        Self {
            with_text: Regex::new(r"\[(.+)]\((.+)\)").unwrap(),
            only_link: Regex::new(r"<(https?://)(.+)>").unwrap(),
        }
    }
}

impl HtmlTransformer for Link {
    fn transform(&self, element: &str) -> String {
        let element = super::sequential_replace(element, &self.with_text, |caps: &Captures|
            build_link(&caps[2], &caps[1]));
        super::sequential_replace(&element, &self.only_link, |caps: &Captures| {
            let url = format!("{}{}", &caps[1], &caps[2]);
            build_link(&url, &url)
        })
    }
}

fn build_link(href: &str, text: &str) -> String {
    format!("<a href=\"{href}\">{text}</a>")
}

#[cfg(test)]
mod tests {
    use super::Link;
    use crate::html::HtmlTransformer;

    #[test]
    fn links_with_text() {
        assert_eq!(
            "Parses <a href=\"https://url\">text</a> links",
            Link::default().transform("Parses [text](https://url) links")
        )
    }

    #[test]
    fn links_without_text() {
        let link = Link::default();
        assert_eq!(
            "Parses secure <a href=\"https://url\">https://url</a>",
            link.transform("Parses secure <https://url>")
        );
        assert_eq!(
            "Parses just http <a href=\"http://url\">http://url</a>",
            link.transform("Parses just http <http://url>")
        )
    }

    #[test]
    fn ignores_only_link_missing_protocol() {
        assert_eq!(
            "Parses just <invalid://url>",
            Link::default().transform("Parses just <invalid://url>")
        )
    }

}