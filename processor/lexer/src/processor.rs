mod empty;
mod metadata;
mod header;
mod markup;

use crate::Token;

pub use empty::Empty;
pub use header::Header;
pub use markup::Markup;
pub use metadata::Metadata;
pub use metadata::MetadataPair;

pub trait LineProcessor {
    fn process_line(&self, line: &str) -> Option<Token>;
}
