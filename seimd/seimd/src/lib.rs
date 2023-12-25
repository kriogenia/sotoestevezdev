use std::collections::HashMap;

pub mod html;
pub(crate) mod line;

pub struct Parsed {
    pub metadata: HashMap<String, String>,
    pub html: String,
}
