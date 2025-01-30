use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use crate::file_patterns::FilePatterns;

/// The set of default ignore patterns
///
/// Importantly, default patterns apply with or without a physical `air.toml`, meaning
/// that we absolutely cannot use globs that only match absolute paths underneath a
/// `root`. We supply [ignore::gitignore::GitignoreBuilder] a `root` of the empty string
/// when builing [DEFAULT_IGNORE_PATTERNS], which means that nothing is stripped from
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
static DEFAULT_IGNORE_PATTERN_NAMES: &[&str] = &[
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

static DEFAULT_IGNORE_PATTERNS: LazyLock<IgnorePatterns> = LazyLock::new(|| {
    IgnorePatterns::try_from_iter(PathBuf::new(), vec![], true)
        .expect("Can create default ignore patterns")
});

#[derive(Debug, Clone)]
pub struct IgnorePatterns(FilePatterns);

impl IgnorePatterns {
    pub(crate) fn try_from_iter<'str, P, I>(
        root: P,
        patterns: I,
        defaults: bool,
    ) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
        I: IntoIterator<Item = &'str str>,
    {
        if defaults {
            // Defaults come first, so user supplied patterns end up taking precedence
            let default_patterns = DEFAULT_IGNORE_PATTERN_NAMES.iter().copied();
            let patterns = default_patterns.chain(patterns);
            Ok(Self(FilePatterns::try_from_iter(root, patterns)?))
        } else {
            Ok(Self(FilePatterns::try_from_iter(root, patterns)?))
        }
    }
}

impl Default for IgnorePatterns {
    /// Default ignore patterns
    ///
    /// Used in the [Default] method of [crate::settings::FormatSettings] to ensure that
    /// virtual `air.toml`s use the default ignore patterns.
    fn default() -> Self {
        DEFAULT_IGNORE_PATTERNS.clone()
    }
}

impl Deref for IgnorePatterns {
    type Target = FilePatterns;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IgnorePatterns {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use crate::settings::ignore_patterns::IgnorePatterns;
    use crate::settings::ignore_patterns::DEFAULT_IGNORE_PATTERN_NAMES;

    #[test]
    fn test_doublestar_default_patterns() {
        let _ = DEFAULT_IGNORE_PATTERN_NAMES
            .iter()
            .map(|pattern| assert!(pattern.starts_with("**/")));
    }

    #[test]
    fn test_default_ignore() -> anyhow::Result<()> {
        let default_patterns = IgnorePatterns::default();

        assert!(default_patterns.matched("renv", true).is_some());
        assert!(default_patterns.matched("renv", false).is_none());
        assert!(default_patterns
            .matched_path_or_any_parents("renv/activate.R", false)
            .is_some());

        assert!(default_patterns.matched("cpp11.R", false).is_some());
        assert!(default_patterns.matched("foo/cpp11.R", false).is_some());

        assert!(default_patterns
            .matched("import-standalone-types-check.R", false)
            .is_some());
        assert!(default_patterns
            .matched("R/import-standalone-foo.R", false)
            .is_some());
        assert!(default_patterns
            .matched("pkg/R/import-standalone-foo.R", false)
            .is_some());

        Ok(())
    }
}
