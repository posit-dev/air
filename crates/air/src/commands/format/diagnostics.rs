use std::io;
use std::path::PathBuf;

use biome_console::fmt::Formatter;
use biome_console::fmt::Termcolor;
use biome_console::markup;
use biome_diagnostics::DiagnosticExt;
use biome_diagnostics::PrintDiagnostic;
use biome_diagnostics::termcolor::WriteColor;
use biome_parser::diagnostic::ParseDiagnostic;
use fs::relativize_path;

/// A file that failed to parse, with everything needed to render its diagnostics
#[derive(Debug)]
pub(crate) struct ParseFailure {
    /// Path shown in each diagnostic header
    pub(crate) path: PathBuf,
    pub(crate) source: String,
    pub(crate) diagnostics: Vec<ParseDiagnostic>,
}

/// Renders every diagnostic of every failure, in order, as code frames
pub(crate) fn print_parse_failures(
    failures: &[ParseFailure],
    writer: &mut impl WriteColor,
) -> io::Result<()> {
    let mut writer = Termcolor(writer);
    let mut formatter = Formatter::new(&mut writer);

    for failure in failures {
        for diagnostic in &failure.diagnostics {
            let diagnostic = diagnostic
                .clone()
                .with_file_path(relativize_path(failure.path.as_path()))
                .with_file_source_code(failure.source.as_str());

            formatter.write_markup(markup! { {PrintDiagnostic::simple(&diagnostic)} })?;
        }
    }

    Ok(())
}
