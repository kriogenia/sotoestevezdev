use crate::html::HtmlTransformer;
use regex::{Captures, Regex};

pub struct Image {
    re: Regex,
}

impl Default for Image {
    fn default() -> Self {
        Self {
            re: Regex::new(r"!\[(.+)]\((.+)\)").unwrap(),
        }
    }
}

impl HtmlTransformer for Image {
    fn transform(&self, element: &str) -> String {
        super::sequential_replace(element, &self.re, |caps: &Captures| {
            format!("<img href=\"{}\" alt=\"{}\"/>", &caps[2], &caps[1])
        })
    }
}

#[cfg(test)]
mod tests {
    use super::Image;
    use crate::html::HtmlTransformer;

    #[test]
    fn image() {
        assert_eq!(
            "Parses <img href=\"https://link.to.img\" alt=\"alternative text\"/> images",
            Image::default().transform("Parses ![alternative text](https://link.to.img) images")
        )
    }
}
