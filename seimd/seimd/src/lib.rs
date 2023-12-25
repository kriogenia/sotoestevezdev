use std::collections::HashMap;

pub(crate) mod html;
pub(crate) mod line;
mod parser;

pub struct Parsed {
    pub metadata: HashMap<String, String>,
    pub html: String,
}
