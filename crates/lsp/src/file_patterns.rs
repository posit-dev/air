use std::path::Path;

use workspace::settings::FormatSettings;
use workspace::settings::IgnorePatterns;

/// Return `true` if the document at the given [`Path`] should be ignored during formatting
pub(crate) fn is_document_ignored_during_formatting(
    path: &Path,
    format_settings: &FormatSettings,
    language_id: String,
) -> bool {
    is_document_ignored(path, &format_settings.ignore, language_id)
}

/// Return `true` if the document at the given [`Path`] should be ignored
///
/// The logic for the resolution considers both inclusion and exclusion and is as follows:
/// 1. Check for `ignore` patterns first
/// 2. Check for `include` patterns next (hypothetical, none right now).
/// 3. Check if the language ID is not R, in which case we ignore.
/// 4. If none of the above conditions are met, the document is not ignored.
fn is_document_ignored(path: &Path, ignore: &IgnorePatterns, language_id: String) -> bool {
    const IS_DIRECTORY: bool = false;

    // Checking ancestors is important. For a path of `renv/activate.R`, we'd miss the
    // default exclusion criteria of the `renv` folder if we don't look up the file tree.
    if let Some(glob) = ignore.matched_path_or_any_parents(path, IS_DIRECTORY) {
        tracing::trace!(
            "Ignored file due to '{glob}' {path:?}",
            glob = glob.original()
        );
        return true;
    }

    if language_id != "r" {
        tracing::trace!("Ignored file due to non-R extension {path:?}");
        return true;
    }

    false
}
