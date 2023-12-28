use crate::provider::Provider;
use regex::Regex;
use std::ops::Range;

pub struct Injector {
    provider: Provider,
    regex: Regex,
    // vec Decorators
}

impl Injector {
    pub fn new(provider: Provider) -> Self {
        Self {
            provider,
            regex: Regex::new(r"[{]{2}\s+(.*)\s+}{2}").unwrap(),
        }
    }

    pub fn inject(&mut self, input: String) -> Result<String, String> {
        let mut buf = String::new();
        let mut pointer = 0;
        for caps in self.regex.captures_iter(input.as_str()) {
            let range = caps.get(0).unwrap().range();
            buf += &&input[pointer..range.start];
            buf += &*self.provider.get(&caps[1])?.html;
            pointer = range.end;
        }
        Ok(buf)
    }
}
