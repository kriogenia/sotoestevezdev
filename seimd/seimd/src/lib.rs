use std::collections::HashMap;

pub(crate) mod html;
pub(crate) mod line;
pub mod parser;

#[derive(Debug)]
pub struct Parsed {
    pub metadata: HashMap<String, String>,
    pub html: String,
}
