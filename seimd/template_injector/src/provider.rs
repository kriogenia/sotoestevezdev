use seimd::parser::SeimdParser;
use seimd::Parsed;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::fs;

pub struct Provider {
    path: String,
    parser: SeimdParser,
    injectables: HashMap<String, Parsed>,
}

impl Provider {
    pub fn new(path: String) -> Self {
        Self {
            path,
            parser: Default::default(),
            injectables: Default::default(),
        }
    }

    pub fn get(&mut self, key: &str) -> Result<&Parsed, String> {
        Ok(match self.injectables.entry(key.to_owned()) {
            Entry::Occupied(value) => value.into_mut(),
            Entry::Vacant(vacant) => {
                let value = load(&self.path, &self.parser, key)?;
                vacant.insert(value)
            }
        })
    }
}

fn load(path: &String, parser: &SeimdParser, file_name: &str) -> Result<Parsed, String> {
    let file_path = format!("{}/{}", path, file_name.trim());
    fs::read_to_string(&file_path)
        .map(|s| parser.parse(s))
        .map_err(|e| format!("Error reading {file_path}: {}", e.kind()))
}

impl Debug for Provider {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Provider({}): [{:?}]", self.path, self.injectables)
    }
}
