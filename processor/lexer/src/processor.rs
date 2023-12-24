mod empty;
mod metadata;

use crate::Token;

pub use empty::Empty;
pub use metadata::Metadata;
pub use metadata::MetadataPair;

pub trait LineProcessor {
    fn process_line(&self, line: &str) -> Option<Token>;
}
