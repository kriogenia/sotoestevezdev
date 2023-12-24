mod metadata_line;
mod metadata_pair;

use crate::Token;

pub use metadata_line::Metadata;
pub use metadata_pair::MetadataPair;

pub trait LineProcessor {
    fn process_line(&self, line: &str) -> Option<Token>;
}
