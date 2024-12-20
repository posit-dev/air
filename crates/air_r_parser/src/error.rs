use biome_diagnostics::Diagnostic;
use biome_parser::prelude::ParseDiagnostic;

/// An error that occurs during parsing
///
/// Simply wraps a `ParseDiagnostic`, mainly so we can implement
/// `std::error::Error` for it, which it oddly does not implement.
#[derive(Debug, Clone)]
pub struct ParseError {
    inner: ParseDiagnostic,
}

impl std::error::Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.diagnostic().description(f)
    }
}

impl ParseError {
    pub fn diagnostic(&self) -> &ParseDiagnostic {
        &self.inner
    }

    pub fn into_diagnostic(self) -> ParseDiagnostic {
        self.inner
    }
}

impl From<ParseDiagnostic> for ParseError {
    fn from(diagnostic: ParseDiagnostic) -> Self {
        Self { inner: diagnostic }
    }
}
