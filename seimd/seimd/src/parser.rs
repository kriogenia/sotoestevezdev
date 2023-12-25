use crate::html::SeimdHtmlTransformer;
use crate::line::SeimdLineProcessor;

#[derive(Default)]
pub struct SeimdParser {
    line_processor: SeimdLineProcessor,
    html_parser: SeimdHtmlTransformer,
}

impl SeimdParser {
    pub fn new() -> Self {
        Default::default()
    }
}