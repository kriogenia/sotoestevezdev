pub mod processor;

#[derive(Debug, PartialEq)]
pub enum Token {
    Metadata,
    MetadataPair(String, String),
    Empty,
    Markup(String),
    Header(u8, String),
}
