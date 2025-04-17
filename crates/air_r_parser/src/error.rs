use biome_parser::prelude::ParseDiagnostic;
use biome_rowan::TextRange;

/// An error that occurs during parsing
///
/// Replacement for [biome_parser::ParseDiagnostic], mainly so we can implement
/// [std::error::Error], which it oddly does not implement.
#[derive(Debug, Clone)]
pub struct ParseError {
    message: String,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl ParseError {
    pub(crate) fn new(message: String) -> Self {
        Self { message }
    }
}

impl From<ParseError> for ParseDiagnostic {
    fn from(error: ParseError) -> Self {
        let span: Option<TextRange> = None;
        ParseDiagnostic::new(error.message, span)
    }
}
