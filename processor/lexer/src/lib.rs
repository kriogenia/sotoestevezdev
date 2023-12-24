mod lexer;
pub(crate) mod processor;

pub use lexer::Lexer;

#[derive(Debug, PartialEq)]
pub enum Token {
    Metadata,
    MetadataPair(String, String),
    Empty,
    Markup(String),
    Header(u8, String),
}
