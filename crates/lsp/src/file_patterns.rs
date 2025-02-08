use std::path::Path;

use workspace::settings::ExcludePatterns;
use workspace::settings::FormatSettings;
use workspace::settings::IncludePatterns;

/// Return `true` if the document at the given [`Path`] should be excluded from formatting
pub(crate) fn is_document_excluded_from_formatting(
    path: &Path,
    format_settings: &FormatSettings,
    language_id: String,
) -> bool {
    is_document_excluded(
        path,
        &format_settings.exclude,
        &format_settings.include,
        language_id,
    )
}

/// Return `true` if the document at the given [`Path`] should be excluded
///
/// The logic for the resolution considers both exclusion and inclusion and is as follows:
/// 1. Check for `exclude` patterns first.
/// 2. Check for `include` patterns next.
/// 3. Check if the language ID is R, in which case we include it. This is a feature
///    unique to language servers.
/// 4. If none of the above conditions are met, the document is excluded.
fn is_document_excluded(
    path: &Path,
    exclude: &ExcludePatterns,
    include: &IncludePatterns,
    language_id: String,
) -> bool {
    const IS_DIRECTORY: bool = false;

    // First check for explicit exclusions.
    // Checking ancestors is important. For a path of `renv/activate.R`, we'd miss the
    // default exclusion criteria of the `renv` folder if we don't look up the file tree.
    if let Some(glob) = exclude.matched_path_or_any_parents(path, IS_DIRECTORY) {
        tracing::trace!(
            "Excluded file due to '{glob}' {path:?}",
            glob = glob.original()
        );
        return true;
    }

    // Then check for explicit inclusions (mostly for `.R` file extensions)
    if let Some(glob) = include.matched_path_or_any_parents(path, IS_DIRECTORY) {
        tracing::trace!(
            "Included file due to '{glob}' {path:?}",
            glob = glob.original()
        );
        return false;
    }

    // Then check if `r` is the language id, which is a feature unique to LSPs
    if language_id == "r" {
        tracing::trace!("Included file due to client provided R language id {path:?}");
        return false;
    }

    tracing::trace!("Excluded file due to fallthrough {path:?}");
    true
}
