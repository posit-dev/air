use air_r_parser::RParserOptions;
use workspace::settings::FormatSettings;

pub(crate) fn format(
    source: &str,
    formatter_settings: &FormatSettings,
) -> crate::Result<Option<String>> {
    let parse = air_r_parser::parse(source, RParserOptions::default());

    if parse.has_errors() {
        return Err(anyhow::anyhow!("Can't format when there are parse errors."));
    }

    // Do we need to check that `doc` is indeed an R file? What about special
    // files that don't have extensions like `NAMESPACE`, do we hard-code a
    // list? What about unnamed temporary files?

    let format_options = formatter_settings.to_format_options(source);
    let formatted = air_r_formatter::format_node(format_options, &parse.syntax())?;
    let code = formatted.print()?.into_code();

    Ok(Some(code))
}

// pub(crate) fn format_range(
//     document: &TextDocument,
//     formatter_settings: &FormatSettings,
//     range: TextRange,
// ) -> crate::Result<Option<PrintedRange>> {
//     let format_options = formatter_settings.to_format_options(source_type, document.contents());
//
//     match ruff_python_formatter::format_range(document.contents(), range, format_options) {
//         Ok(formatted) => {
//             if formatted.as_code() == document.contents() {
//                 Ok(None)
//             } else {
//                 Ok(Some(formatted))
//             }
//         }
//         // Special case - syntax/parse errors are handled here instead of
//         // being propagated as visible server errors.
//         Err(FormatModuleError::ParseError(error)) => {
//             tracing::warn!("Unable to format document range: {error}");
//             Ok(None)
//         }
//         Err(err) => Err(err.into()),
//     }
// }
