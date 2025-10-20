use std::path::Path;
use std::sync::LazyLock;

use ignore::gitignore::Glob;

use crate::file_patterns::DefaultFilePatterns;

/// The set of default exclude patterns
///
/// Importantly, default patterns apply with or without a physical `air.toml`, meaning
/// that we absolutely cannot use globs that only match absolute paths underneath a
/// `root`. We supply [ignore::gitignore::GitignoreBuilder] a `root` of the empty string
/// when builing [DEFAULT_EXCLUDE_PATTERNS], which means that nothing is stripped from
/// paths by `ignore` before performing a match (which is why the patterns can't match
/// absolute paths).
///
/// In a default pattern, you cannot use:
/// - Preceding `/`, like `/renv`, as that only matches `{root}/renv`
/// - Middle `/`, like `renv/*.R`, as that only matches `{root}/renv/*.R`
///
/// While not strictly necessary, to easily enforce this in tests all default patterns
/// must start with `**/`. Note that [ignore::gitignore::GitignoreBuilder] ensures that
/// `.git/` is equivalent to `**/.git/` and `cpp11.R` is equivalent to `**/cpp11.R`, so
/// this prefixing happens eventually anyways.
static DEFAULT_EXCLUDE_PATTERN_NAMES: &[&str] = &[
    // Directories
    // The trailing `/` prevents matching a non-directory file named, for example, `renv`.
    "**/.git/",
    "**/renv/",
    "**/revdep/",
    // Files
    "**/cpp11.R",
    "**/RcppExports.R",
    "**/extendr-wrappers.R",
    "**/import-standalone-*.R",
];

static DEFAULT_EXCLUDE_PATTERNS: LazyLock<DefaultFilePatterns> = LazyLock::new(|| {
    DefaultFilePatterns::try_from_iter(DEFAULT_EXCLUDE_PATTERN_NAMES.iter().copied())
        .expect("Can create default exclude patterns")
});

/// Typed wrapper around [DEFAULT_EXCLUDE_PATTERNS]
///
/// Allows for free creation of [DefaultExcludePatterns] structs without needing to clone
/// the global [DEFAULT_EXCLUDE_PATTERNS] object.
#[derive(Debug)]
pub struct DefaultExcludePatterns;

impl DefaultExcludePatterns {
    pub fn matched<P>(&self, path: P, is_directory: bool) -> Option<&Glob>
    where
        P: AsRef<Path>,
    {
        DEFAULT_EXCLUDE_PATTERNS.matched(path, is_directory)
    }

    pub fn matched_path_or_any_parents<P>(&self, path: P, is_directory: bool) -> Option<&Glob>
    where
        P: AsRef<Path>,
    {
        DEFAULT_EXCLUDE_PATTERNS.matched_path_or_any_parents(path, is_directory)
    }
}

#[cfg(test)]
mod test {
    use crate::settings::default_exclude_patterns::DEFAULT_EXCLUDE_PATTERN_NAMES;
    use crate::settings::default_exclude_patterns::DefaultExcludePatterns;

    #[test]
    fn test_doublestar_default_patterns() {
        let _ = DEFAULT_EXCLUDE_PATTERN_NAMES
            .iter()
            .map(|pattern| assert!(pattern.starts_with("**/")));
    }

    #[test]
    fn test_default_exclude() -> anyhow::Result<()> {
        let default_patterns = DefaultExcludePatterns;

        assert!(default_patterns.matched("renv", true).is_some());
        assert!(default_patterns.matched("renv", false).is_none());
        assert!(
            default_patterns
                .matched_path_or_any_parents("renv/activate.R", false)
                .is_some()
        );

        assert!(default_patterns.matched("cpp11.R", false).is_some());
        assert!(default_patterns.matched("foo/cpp11.R", false).is_some());

        assert!(
            default_patterns
                .matched("import-standalone-types-check.R", false)
                .is_some()
        );
        assert!(
            default_patterns
                .matched("R/import-standalone-foo.R", false)
                .is_some()
        );
        assert!(
            default_patterns
                .matched("pkg/R/import-standalone-foo.R", false)
                .is_some()
        );

        Ok(())
    }
}
