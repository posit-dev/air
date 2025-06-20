use std::ops::Deref;
use std::ops::DerefMut;
use std::sync::LazyLock;

use crate::file_patterns::DefaultFilePatterns;

/// The set of default include patterns
///
/// See `DEFAULT_EXCLUDE_PATTERN_NAMES` for details on the exact structure of what can
/// be supplied here.
static DEFAULT_INCLUDE_PATTERN_NAMES: &[&str] = &[
    // R files with any filename at any depth
    "**/*.[R,r]",
];

static DEFAULT_INCLUDE_PATTERNS: LazyLock<DefaultIncludePatterns> = LazyLock::new(|| {
    DefaultIncludePatterns(
        DefaultFilePatterns::try_from_iter(DEFAULT_INCLUDE_PATTERN_NAMES.iter().copied())
            .expect("Can create default include patterns"),
    )
});

#[derive(Debug, Clone)]
pub struct DefaultIncludePatterns(DefaultFilePatterns);

impl Default for DefaultIncludePatterns {
    /// Default include patterns
    ///
    /// Used in the [Default] method of [crate::settings::FormatSettings] to ensure that
    /// virtual `air.toml`s use the default include patterns.
    fn default() -> Self {
        DEFAULT_INCLUDE_PATTERNS.clone()
    }
}

impl Deref for DefaultIncludePatterns {
    type Target = DefaultFilePatterns;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for DefaultIncludePatterns {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
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
        let default_patterns = DefaultIncludePatterns::default();

        assert!(default_patterns.matched("cpp11.R", false).is_some());
        assert!(default_patterns.matched("foo/cpp11.R", false).is_some());

        assert!(default_patterns.matched("cpp11.r", false).is_some());
        assert!(default_patterns.matched("foo/cpp11.r", false).is_some());

        assert!(default_patterns.matched("cpp11.py", false).is_none());
        assert!(default_patterns.matched("foo/cpp11.py", false).is_none());

        Ok(())
    }
}
