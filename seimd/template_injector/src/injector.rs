use crate::decoration::HtmlDecorator;
use crate::provider::Provider;
use regex::Regex;

pub struct Injector {
    provider: Provider,
    regex: Regex,
    decorators: Vec<HtmlDecorator>,
}

impl Injector {
    pub fn new(provider: Provider) -> Self {
        Self {
            provider,
            regex: Regex::new(r"[{]{2}\s+(.*)\s+}{2}").unwrap(),
            decorators: vec![HtmlDecorator::Legend, HtmlDecorator::ParentWrapper],
        }
    }

    pub fn inject(&mut self, input: String) -> Result<String, String> {
        let mut buf = String::new();
        let mut pointer = 0;
        for caps in self.regex.captures_iter(input.as_str()) {
            let range = caps.get(0).unwrap().range();

            let parsed = &*self.provider.get(&caps[1])?;
            let html = self.decorators
                .iter()
                .fold(parsed.html.clone(), |acc, dec| dec.decorate(parsed, acc));

            let prev = &input[pointer..range.start];
            pointer = range.end;

            buf = format!("{buf}{prev}{html}")
        }
        Ok(buf + &input[pointer..])
    }

}