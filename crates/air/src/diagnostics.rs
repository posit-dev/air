use std::io;
use std::ops::Range;
use std::path::Path;
use std::path::PathBuf;

use biome_console::Console;
use biome_console::ConsoleExt;
use biome_console::DebugDisplay;
use biome_console::fmt::Display;
use biome_console::markup;
use biome_diagnostics::Advices;
use biome_diagnostics::Diagnostic;
use biome_diagnostics::DiagnosticExt;
use biome_diagnostics::Error;
use biome_diagnostics::MessageAndDescription;
use biome_diagnostics::PrintDiagnostic;
use biome_diagnostics::Resource;
use biome_diagnostics::Severity;
use biome_diagnostics::Visit;
use biome_parser::diagnostic::ParseDiagnostic;
use fs::relativize_path;
use workspace::discovery::DiscoverSettingsError;

/// Prints `diagnostics` sorted by path, followed by a count of the errors
pub(crate) fn print_format_diagnostics(console: &mut dyn Console, mut diagnostics: Vec<Error>) {
    // Discovery runs in parallel so order isn't deterministic. The sort is stable
    // to keep each file's diagnostics in source order.
    diagnostics.sort_by(|lhs, rhs| diagnostic_path(lhs).cmp(&diagnostic_path(rhs)));

    for diagnostic in &diagnostics {
        console.error(markup! {{PrintDiagnostic::simple(diagnostic)}});
    }

    let errors = diagnostics
        .iter()
        .filter(|diagnostic| diagnostic.severity() >= Severity::Error)
        .count();

    if errors > 0 {
        let summary = FormatSummaryDiagnostic::new(errors);
        console.error(markup! {{PrintDiagnostic::simple(&summary)}});
    }
}

/// Prints a single `format --check` diagnostic, listing all `paths`, sorted
pub(crate) fn print_format_check_diagnostic(console: &mut dyn Console, paths: &[PathBuf]) {
    if !paths.is_empty() {
        let summary = FormatCheckSummaryDiagnostic::new(paths);
        console.error(markup! {{PrintDiagnostic::simple(&summary)}});
    }
}

fn diagnostic_path(diagnostic: &Error) -> Option<&Path> {
    match diagnostic.location().resource {
        Some(Resource::File(path)) => Some(Path::new(path)),
        Some(Resource::Argv) => None,
        Some(Resource::Memory) => None,
        None => None,
    }
}

/// Attaches `path` and `source` to each syntax error, so they render as code frames
pub(crate) fn from_parse_diagnostics(
    path: &Path,
    source: &str,
    diagnostics: Vec<ParseDiagnostic>,
) -> Vec<Error> {
    let path = relativize_path(path);

    diagnostics
        .into_iter()
        .map(|diagnostic| {
            diagnostic
                .with_file_path(path.clone())
                .with_file_source_code(source.to_string())
        })
        .collect()
}

/// Turns an `air.toml` error into a diagnostic, with a code frame when the TOML
/// error has a span (like with a syntax error)
pub(crate) fn from_discover_settings_error(error: DiscoverSettingsError) -> Error {
    match error {
        DiscoverSettingsError::Read { path, error } => ReadDiagnostic::new(&path, error).into(),
        DiscoverSettingsError::Deserialize {
            path,
            contents,
            error,
        } => SettingsDiagnostic::new(
            &path,
            error.message().to_string(),
            error.span(),
            Some(contents),
        )
        .into(),
        DiscoverSettingsError::Logic { path, error } => {
            SettingsDiagnostic::new(&path, error.to_string(), None, None).into()
        }
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "internalError/io",
    severity = Error,
    message(
        description = "Failed to read file: {error}",
        message("Failed to read file: "{DebugDisplay(&self.error)})
    )
)]
pub(crate) struct ReadDiagnostic {
    #[location(resource)]
    path: String,
    error: io::Error,
}

impl ReadDiagnostic {
    pub(crate) fn new(path: &Path, error: io::Error) -> Self {
        Self {
            path: relativize_path(path),
            error,
        }
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "internalError/io",
    severity = Error,
    message(
        description = "Failed to write file: {error}",
        message("Failed to write file: "{DebugDisplay(&self.error)})
    )
)]
pub(crate) struct WriteDiagnostic {
    #[location(resource)]
    path: String,
    error: io::Error,
}

impl WriteDiagnostic {
    pub(crate) fn new(path: &Path, error: io::Error) -> Self {
        Self {
            path: relativize_path(path),
            error,
        }
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(category = "configuration", severity = Error)]
struct SettingsDiagnostic {
    #[location(resource)]
    path: String,
    #[message]
    #[description]
    message: MessageAndDescription,
    #[location(span)]
    span: Option<Range<usize>>,
    #[location(source_code)]
    contents: Option<String>,
}

impl SettingsDiagnostic {
    fn new(
        path: &Path,
        description: String,
        span: Option<Range<usize>>,
        contents: Option<String>,
    ) -> Self {
        Self {
            path: relativize_path(path),
            message: MessageAndDescription::from(description),
            span,
            contents,
        }
    }
}

/// A [biome_formatter::FormatError] or [biome_formatter::PrintError], which only
/// happen when Air's formatter has a bug
#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "format",
    severity = Error,
    message(
        description = "Failed to format file: {error}",
        message("Failed to format file: "{DebugDisplay(&self.error)})
    )
)]
pub(crate) struct FormatDiagnostic {
    #[location(resource)]
    path: String,
    error: Box<dyn std::error::Error + Send + Sync>,
}

impl FormatDiagnostic {
    pub(crate) fn new(
        path: &Path,
        error: impl Into<Box<dyn std::error::Error + Send + Sync>>,
    ) -> Self {
        Self {
            path: relativize_path(path),
            error: error.into(),
        }
    }
}

/// An error from walking the requested paths, like a missing file or an
/// unreadable directory
#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "internalError/fs",
    severity = Error,
    message(
        description = "{error}",
        message({DebugDisplay(&self.error)})
    )
)]
pub(crate) struct WalkDiagnostic {
    #[location(resource)]
    path: Option<String>,
    error: ignore::Error,
}

impl WalkDiagnostic {
    pub(crate) fn new(error: ignore::Error) -> Self {
        let (path, error) = unwrap_walk_error(error, None);

        Self {
            path: path.map(relativize_path),
            error,
        }
    }
}

/// Peels the path and depth that `ignore` wraps around an error
fn unwrap_walk_error(
    error: ignore::Error,
    path: Option<PathBuf>,
) -> (Option<PathBuf>, ignore::Error) {
    match error {
        ignore::Error::WithPath { path: inner, err } => {
            unwrap_walk_error(*err, path.or(Some(inner)))
        }
        ignore::Error::WithDepth { err, .. } => unwrap_walk_error(*err, path),
        ignore::Error::Partial(..) => (path, error),
        ignore::Error::WithLineNumber { .. } => (path, error),
        ignore::Error::Loop { .. } => (path, error),
        ignore::Error::Io(..) => (path, error),
        ignore::Error::Glob { .. } => (path, error),
        ignore::Error::UnrecognizedFileType(..) => (path, error),
        ignore::Error::InvalidDefinition => (path, error),
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "stdin",
    severity = Error,
    message(
        description = "Failed to read from stdin: {error}",
        message("Failed to read from stdin: "{DebugDisplay(&self.error)})
    )
)]
pub(crate) struct StdinReadDiagnostic {
    error: io::Error,
}

impl StdinReadDiagnostic {
    pub(crate) fn new(error: io::Error) -> Self {
        Self { error }
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "stdin",
    severity = Error,
    message(
        description = "Failed to write to stdout: {error}",
        message("Failed to write to stdout: "{DebugDisplay(&self.error)})
    )
)]
pub(crate) struct StdoutWriteDiagnostic {
    error: io::Error,
}

impl StdoutWriteDiagnostic {
    pub(crate) fn new(error: io::Error) -> Self {
        Self { error }
    }
}

#[derive(Debug, Diagnostic)]
#[diagnostic(
    category = "flags/invalid",
    severity = Error,
    message(
        description = "Can't supply paths when reading from stdin: {paths}",
        message("Can't supply paths when reading from stdin: "{self.paths})
    )
)]
pub(crate) struct StdinWithPathsDiagnostic {
    paths: String,
}

impl StdinWithPathsDiagnostic {
    pub(crate) fn new(paths: &[PathBuf]) -> Self {
        let paths = paths
            .iter()
            .map(|path| format!("'{path}'", path = path.display()))
            .collect::<Vec<String>>()
            .join(",");

        Self { paths }
    }
}

/// Summary report of `format` errors
#[derive(Debug, Diagnostic)]
#[diagnostic(category = "format", severity = Error)]
struct FormatSummaryDiagnostic {
    #[message]
    #[description]
    message: MessageAndDescription,
}

impl FormatSummaryDiagnostic {
    fn new(count: usize) -> Self {
        let message = match count {
            1 => String::from("Found 1 error."),
            count => format!("Found {count} errors."),
        };

        Self {
            message: MessageAndDescription::from(message),
        }
    }
}

/// Summary report of `format --check` files that would be reformatted
#[derive(Debug, Diagnostic)]
#[diagnostic(category = "format", severity = Error)]
struct FormatCheckSummaryDiagnostic {
    #[message]
    #[description]
    message: MessageAndDescription,
    #[advice]
    paths: FormatCheckPaths,
}

impl FormatCheckSummaryDiagnostic {
    fn new(paths: &[PathBuf]) -> Self {
        let message = match paths.len() {
            1 => String::from("1 file would be reformatted."),
            count => format!("{count} files would be reformatted."),
        };

        // Relativize and sort since discovery runs in parallel and isn't stable
        let mut paths: Vec<String> = paths.iter().map(relativize_path).collect();
        paths.sort_by(|lhs, rhs| Path::new(lhs).cmp(Path::new(rhs)));

        Self {
            message: MessageAndDescription::from(message),
            paths: FormatCheckPaths(paths),
        }
    }
}

#[derive(Debug)]
struct FormatCheckPaths(Vec<String>);

impl Advices for FormatCheckPaths {
    fn record(&self, visitor: &mut dyn Visit) -> io::Result<()> {
        // Emits the bullets of the paths that would be reformatted, like:
        //
        // ```
        // x 2 files would be reformatted.
        //
        // - a.R <- these
        // - b.R
        // ```
        let paths: Vec<&dyn Display> = self.0.iter().map(|path| path as &dyn Display).collect();
        visitor.record_list(&paths)
    }
}

#[cfg(test)]
mod test {
    use std::io;
    use std::path::Path;
    use std::path::PathBuf;

    use biome_diagnostics::PrintDescription;

    use crate::diagnostics::ReadDiagnostic;
    use crate::diagnostics::StdinWithPathsDiagnostic;

    #[test]
    fn test_description_is_rendered_from_message() {
        let diagnostic = ReadDiagnostic::new(
            Path::new("test.R"),
            io::Error::new(io::ErrorKind::InvalidData, "bad bytes"),
        );
        assert_eq!(
            PrintDescription(&diagnostic).to_string(),
            "Failed to read file: bad bytes"
        );

        let diagnostic = StdinWithPathsDiagnostic::new(&[PathBuf::from("a.R")]);
        assert_eq!(
            PrintDescription(&diagnostic).to_string(),
            "Can't supply paths when reading from stdin: 'a.R'"
        );
    }
}
