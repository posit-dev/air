use air_r_formatter::context::RFormatOptions;
use air_r_parser::RParserOptions;
use biome_parser::diagnostic::ParseDiagnostic;

#[derive(Debug)]
pub(crate) enum FormattedSource {
    /// The source was formatted, the [`String`] contains the transformed source code.
    Changed(String),
    /// The source was unchanged.
    Unchanged,
}

// We don't actually derive `Error`, as this is just a passthrough type that
// we deconstruct into a real error type at the call site
#[derive(Debug)]
pub(crate) enum FormatSourceError {
    Parse(Vec<ParseDiagnostic>),
    Format(biome_formatter::FormatError),
    Print(biome_formatter::PrintError),
}

pub(crate) fn format_source(
    source: &str,
    format_options: RFormatOptions,
) -> Result<FormattedSource, FormatSourceError> {
    let parse = air_r_parser::parse(source, RParserOptions::default());

    if parse.has_errors() {
        return Err(FormatSourceError::Parse(parse.into_diagnostics()));
    }

    let formatted = air_r_formatter::format_node(format_options, &parse.syntax())
        .map_err(FormatSourceError::Format)?;
    let formatted = formatted.print().map_err(FormatSourceError::Print)?;
    let formatted = formatted.into_code();

    if formatted == source {
        Ok(FormattedSource::Unchanged)
    } else {
        Ok(FormattedSource::Changed(formatted))
    }
}
