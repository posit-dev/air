use std::path::Path;

use workspace::settings::FormatSettings;
use workspace::settings::IgnorePatterns;
use workspace::settings::IncludePatterns;

/// Return `true` if the document at the given [`Path`] should be ignored during formatting
pub(crate) fn is_document_ignored_during_formatting(
    path: &Path,
    format_settings: &FormatSettings,
    language_id: String,
) -> bool {
    is_document_ignored(
        path,
        &format_settings.ignore,
        &format_settings.include,
        language_id,
    )
}

/// Return `true` if the document at the given [`Path`] should be ignored
///
/// The logic for the resolution considers both inclusion and exclusion and is as follows:
/// 1. Check for `ignore` patterns first.
/// 2. Check for `include` patterns next.
/// 3. Check if the language ID is R, in which case we include it. This is a feature
///    unique to language servers.
/// 4. If none of the above conditions are met, the document is excluded.
fn is_document_ignored(
    path: &Path,
    ignore: &IgnorePatterns,
    include: &IncludePatterns,
    language_id: String,
) -> bool {
    const IS_DIRECTORY: bool = false;

    // First check for explicit exclusions.
    // Checking ancestors is important. For a path of `renv/activate.R`, we'd miss the
    // default exclusion criteria of the `renv` folder if we don't look up the file tree.
    if let Some(glob) = ignore.matched_path_or_any_parents(path, IS_DIRECTORY) {
        tracing::trace!(
            "Ignored file due to '{glob}' {path:?}",
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
