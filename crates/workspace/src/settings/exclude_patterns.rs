use std::path::Path;

use ignore::gitignore::Glob;

use crate::file_patterns::RootedFilePatterns;
use crate::file_patterns::UnrootedFilePatterns;

/// User supplied `exclude` patterns
///
/// The presence of a `root` at construction time determines the variant:
/// - A project `air.toml` roots its patterns at the folder it lives in, giving
///   [ExcludePatterns::Rooted].
/// - A user level `air.toml` has no project directory to root against, giving
///   [ExcludePatterns::Unrooted], whose patterns must each match at any depth.
#[derive(Debug, Clone)]
pub enum ExcludePatterns {
    Rooted(RootedFilePatterns),
    Unrooted(UnrootedFilePatterns),
}

impl ExcludePatterns {
    /// Construct [ExcludePatterns] from an iterator of patterns
    ///
    /// A `root` of `Some` roots the patterns at that directory. A `root` of `None` means
    /// there is no directory to root against, so every pattern must match at any depth,
    /// otherwise an error is thrown on creation.
    pub(crate) fn try_from_iter<'str, I>(root: Option<&Path>, patterns: I) -> anyhow::Result<Self>
    where
        I: IntoIterator<Item = &'str str>,
    {
        match root {
            Some(root) => Ok(Self::Rooted(RootedFilePatterns::try_from_iter(
                root, patterns,
            )?)),
            None => Ok(Self::Unrooted(UnrootedFilePatterns::try_from_iter(
                patterns,
            )?)),
        }
    }

    /// Returns the glob that matches this `path`, or `None` if no glob matches
    pub(crate) fn matched<P>(&self, path: P, is_directory: bool) -> Option<&Glob>
    where
        P: AsRef<Path>,
    {
        match self {
            Self::Rooted(patterns) => patterns.matched(path, is_directory),
            Self::Unrooted(patterns) => patterns.matched(path, is_directory),
        }
    }

    /// Returns the glob that matches this `path` or any parent, or `None` if no glob
    /// matches
    pub fn matched_path_or_any_parents<P>(&self, path: P, is_directory: bool) -> Option<&Glob>
    where
        P: AsRef<Path>,
    {
        match self {
            Self::Rooted(patterns) => patterns.matched_path_or_any_parents(path, is_directory),
            Self::Unrooted(patterns) => patterns.matched_path_or_any_parents(path, is_directory),
        }
    }
}
