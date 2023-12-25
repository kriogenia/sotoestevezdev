use std::collections::HashMap;

pub(crate) mod line;
pub mod html;

pub struct Parsed {
    pub metadata: HashMap<String, String>,
    pub html: String,
}