mod bold;

use regex::{Captures, Regex};

pub trait HtmlTransformer {
    fn transform(&self, element: &str) -> String;
}

fn sequential_replace(element: &str, re: &Regex, replacer: fn(&Captures) -> String) -> String {
    let mut remaining = &element[..];
    let mut acc = "".to_string();
    if re.is_match(element) {
        while let Some(offset) = re.shortest_match(remaining) {
            acc += &*re.replace(&remaining[..offset], replacer);
            remaining = &remaining[offset..];
        }
        acc
    } else { element.to_string() }
}
