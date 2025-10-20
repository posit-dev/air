use std::path::Path;
use std::sync::LazyLock;

use ignore::gitignore::Glob;

use crate::file_patterns::DefaultFilePatterns;

/// The set of default include patterns
///
/// See `DEFAULT_EXCLUDE_PATTERN_NAMES` for details on the exact structure of what can
/// be supplied here.
static DEFAULT_INCLUDE_PATTERN_NAMES: &[&str] = &[
    // R files with any filename at any depth
    "**/*.[R,r]",
];

static DEFAULT_INCLUDE_PATTERNS: LazyLock<DefaultFilePatterns> = LazyLock::new(|| {
    DefaultFilePatterns::try_from_iter(DEFAULT_INCLUDE_PATTERN_NAMES.iter().copied())
        .expect("Can create default include patterns")
});

/// Typed wrapper around [DEFAULT_INCLUDE_PATTERNS]
///
/// Allows for free creation of [DefaultIncludePatterns] structs without needing to clone
/// the global [DEFAULT_INCLUDE_PATTERNS] object.
#[derive(Debug)]
pub struct DefaultIncludePatterns;

impl DefaultIncludePatterns {
    pub fn matched<P>(&self, path: P, is_directory: bool) -> Option<&Glob>
    where
        P: AsRef<Path>,
    {
        DEFAULT_INCLUDE_PATTERNS.matched(path, is_directory)
    }

    pub fn matched_path_or_any_parents<P>(&self, path: P, is_directory: bool) -> Option<&Glob>
    where
        P: AsRef<Path>,
    {
        DEFAULT_INCLUDE_PATTERNS.matched_path_or_any_parents(path, is_directory)
    }
}

#[cfg(test)]
mod test {
    use crate::settings::default_include_patterns::DEFAULT_INCLUDE_PATTERN_NAMES;
    use crate::settings::default_include_patterns::DefaultIncludePatterns;

    #[test]
    fn test_doublestar_default_patterns() {
        let _ = DEFAULT_INCLUDE_PATTERN_NAMES
            .iter()
            .map(|pattern| assert!(pattern.starts_with("**/")));
    }

    #[test]
    fn test_default_include() -> anyhow::Result<()> {
        let default_patterns = DefaultIncludePatterns;

        assert!(default_patterns.matched("cpp11.R", false).is_some());
        assert!(default_patterns.matched("foo/cpp11.R", false).is_some());

        assert!(default_patterns.matched("cpp11.r", false).is_some());
        assert!(default_patterns.matched("foo/cpp11.r", false).is_some());

        assert!(default_patterns.matched("cpp11.py", false).is_none());
        assert!(default_patterns.matched("foo/cpp11.py", false).is_none());

        Ok(())
    }
}
