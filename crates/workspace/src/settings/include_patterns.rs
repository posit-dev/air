use std::ops::Deref;
use std::ops::DerefMut;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use crate::file_patterns::FilePatterns;

/// The set of default include patterns
///
/// See `DEFAULT_EXCLUDE_PATTERN_NAMES` for details on the exact structure of what can
/// be supplied here.
static DEFAULT_INCLUDE_PATTERN_NAMES: &[&str] = &[
    // R files with any filename at any depth
    "**/*.[R,r]",
];

static DEFAULT_INCLUDE_PATTERNS: LazyLock<IncludePatterns> = LazyLock::new(|| {
    IncludePatterns::try_from_iter(PathBuf::new(), vec![], true)
        .expect("Can create default include patterns")
});

#[derive(Debug, Clone)]
pub struct IncludePatterns(FilePatterns);

impl IncludePatterns {
    fn try_from_iter<'str, P, I>(root: P, patterns: I, defaults: bool) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
        I: IntoIterator<Item = &'str str>,
    {
        if defaults {
            // Defaults come first, so user supplied patterns end up taking precedence
            let default_patterns = DEFAULT_INCLUDE_PATTERN_NAMES.iter().copied();
            let patterns = default_patterns.chain(patterns);
            Ok(Self(FilePatterns::try_from_iter(root, patterns)?))
        } else {
            Ok(Self(FilePatterns::try_from_iter(root, patterns)?))
        }
    }
}

impl Default for IncludePatterns {
    /// Default include patterns
    ///
    /// Used in the [Default] method of [crate::settings::FormatSettings] to ensure that
    /// virtual `air.toml`s use the default include patterns.
    fn default() -> Self {
        DEFAULT_INCLUDE_PATTERNS.clone()
    }
}

impl Deref for IncludePatterns {
    type Target = FilePatterns;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for IncludePatterns {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[cfg(test)]
mod test {
    use crate::settings::include_patterns::IncludePatterns;
    use crate::settings::include_patterns::DEFAULT_INCLUDE_PATTERN_NAMES;

    #[test]
    fn test_doublestar_default_patterns() {
        let _ = DEFAULT_INCLUDE_PATTERN_NAMES
            .iter()
            .map(|pattern| assert!(pattern.starts_with("**/")));
    }

    #[test]
    fn test_default_include() -> anyhow::Result<()> {
        let default_patterns = IncludePatterns::default();

        assert!(default_patterns.matched("cpp11.R", false).is_some());
        assert!(default_patterns.matched("foo/cpp11.R", false).is_some());

        assert!(default_patterns.matched("cpp11.r", false).is_some());
        assert!(default_patterns.matched("foo/cpp11.r", false).is_some());

        assert!(default_patterns.matched("cpp11.py", false).is_none());
        assert!(default_patterns.matched("foo/cpp11.py", false).is_none());

        Ok(())
    }
}
