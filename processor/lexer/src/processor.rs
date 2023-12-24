mod empty;
mod metadata;
mod header;

use crate::Token;

pub use empty::Empty;
pub use header::Header;
pub use metadata::Metadata;
pub use metadata::MetadataPair;

pub trait LineProcessor {
    fn process_line(&self, line: &str) -> Option<Token>;
}
