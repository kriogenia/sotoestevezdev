use crate::decoration::HtmlDecorator;
use crate::provider::Provider;
use regex::Regex;

pub struct Injector {
    metadata_re: Regex,
    file_re: Regex,
    provider: Provider,
    decorators: Vec<HtmlDecorator>,
}

impl Injector {
    pub fn new(provider: Provider, decorators: Vec<HtmlDecorator>) -> Self {
        Self {
            metadata_re: Regex::new(r"[{]{2}\s+(?<file>.*)\[{2}(?<key>.*)]{2}\s+}{2}").unwrap(),
            file_re: Regex::new(r"[{]{2}\s+(?<file>.*)\s+}{2}").unwrap(),
            provider,
            decorators,
        }
    }

    pub fn inject(&mut self, input: String) -> Result<String, String> {
        self.replace_metadata(&input)
            .and_then(|input| self.replace_html(&input))
    }

    fn replace_metadata(&mut self, input: &str) -> Result<String, String> {
        let mut buf = String::new();
        let mut pointer = 0;

        for caps in self.metadata_re.captures_iter(input) {
            let range = caps.get(0).unwrap().range();

            let parsed = self.provider.get(&caps["file"])?;
            let content = parsed.metadata.get(&caps["key"]);

            let prev = &input[pointer..range.start];
            pointer = range.end;

            buf = format!("{buf}{prev}{}", content.unwrap_or(&String::new()))
        }

        Ok(buf + &input[pointer..])
    }

    fn replace_html(&mut self, input: &str) -> Result<String, String> {
        let mut pointer = 0;
        let mut buf = String::new();

        for caps in self.file_re.captures_iter(input) {
            let range = caps.get(0).unwrap().range();

            let parsed = self.provider.get(&caps[1])?;
            let html = self
                .decorators
                .iter()
                .fold(parsed.html.clone(), |acc, dec| dec.decorate(parsed, acc));

            let prev = &input[pointer..range.start];
            pointer = range.end;

            buf = format!("{buf}{prev}{html}")
        }

        Ok(buf + &input[pointer..])
    }
}
