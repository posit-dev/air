use std::path::Path;
use std::path::PathBuf;

use ignore::gitignore::Gitignore;
use ignore::gitignore::GitignoreBuilder;
use ignore::gitignore::Glob;
use ignore::Match;

/// Matcher for globs that follow a `.gitignore` style
///
/// When constructing the matcher, you supply a `root` path along with the `patterns`
/// to be included in the matcher. When [FilePatterns::matched] is called, the `root`
/// path is always stripped from `path` before matching is done. This ensures that users
/// can specify `"/special.R"` in their `air.toml` to match only `{root}/special.R`, and
/// not also `{root}/subdir/special.R`.
#[derive(Clone, Debug)]
pub struct FilePatterns {
    matcher: Gitignore,
}

/// Matcher for a default set of globs
///
/// Compared to [FilePatterns], [DefaultFilePatterns] is special because it does not
/// allow specification of a `root` path. When constructing [crate::settings::Settings]
/// for a "virtual" `air.toml`, there is no `root` path, but we still want to respect
/// default includes and excludes. To ensure this works correctly, we have to make two
/// main changes:
///
/// - All `patterns` must start with `**/` so they don't depend on the `root`, this is
///   enforced by [DefaultFilePatterns::try_from_iter] at creation time.
///
/// - [DefaultFilePatterns::matched_path_or_any_parents] is custom, rather than relying
///   on [Gitignore] directly. The reason we do this is because [Gitignore]'s version
///   will panic if the `path` we provide contains a root component after `root` stripping
///   has occurred (like a leading `/` on Unix, or leading `C:/` on Windows). We don't
///   have a `root` to strip, and our globs don't depend on `root`, so we don't need this
///   restriction.
#[derive(Clone, Debug)]
pub struct DefaultFilePatterns {
    matcher: Gitignore,
}

impl FilePatterns {
    /// Construct [FilePatterns] from an iterator of patterns
    pub(crate) fn try_from_iter<'str, P, I>(root: P, patterns: I) -> anyhow::Result<Self>
    where
        P: AsRef<Path>,
        I: IntoIterator<Item = &'str str>,
    {
        let mut builder = GitignoreBuilder::new(root);

        for pattern in patterns {
            builder.add_line(None, pattern)?;
        }

        Ok(Self {
            matcher: builder.build()?,
        })
    }

    /// Returns the glob that matches this `path`, or `None` if no glob matches
    ///
    /// We consider a whitelisted file to be `None`, i.e. if `"!file.R"` is supplied, then
    /// we effectively treat that as if we weren't matched at all. We don't advertise the
    /// whitelisting feature though, so this also should not come up much.
    pub(crate) fn matched<P>(&self, path: P, is_directory: bool) -> Option<&Glob>
    where
        P: AsRef<Path>,
    {
        match self.matcher.matched(path, is_directory) {
            Match::None => None,
            Match::Whitelist(_) => None,
            Match::Ignore(glob) => Some(glob),
        }
    }

    /// Returns the glob that matches this `path` or any parent, or `None` if no glob
    /// matches
    ///
    /// More expensive than [FilePatterns::matched], but is required in the LSP where you
    /// don't recursively search a directory, but are instead handed a single file at a
    /// time.
    pub fn matched_path_or_any_parents<P>(&self, path: P, is_directory: bool) -> Option<&Glob>
    where
        P: AsRef<Path>,
    {
        match self.matcher.matched_path_or_any_parents(path, is_directory) {
            Match::None => None,
            Match::Whitelist(_) => None,
            Match::Ignore(glob) => Some(glob),
        }
    }
}

impl DefaultFilePatterns {
    /// Construct [DefaultFilePatterns] from an iterator of patterns
    ///
    /// Note:
    /// - Uses an empty string for the `root`.
    /// - Enforces that all patterns start with `**/`, since we don't have a `root`.
    pub(crate) fn try_from_iter<'str, I>(patterns: I) -> anyhow::Result<Self>
    where
        I: IntoIterator<Item = &'str str>,
    {
        // For the *default* case, the `root` path is always the empty string
        let root = PathBuf::new();

        let mut builder = GitignoreBuilder::new(root);

        for pattern in patterns {
            debug_assert!(pattern.starts_with("**/"));
            builder.add_line(None, pattern)?;
        }

        Ok(Self {
            matcher: builder.build()?,
        })
    }

    /// Returns the glob that matches this `path`, or `None` if no glob matches
    pub(crate) fn matched<P>(&self, path: P, is_directory: bool) -> Option<&Glob>
    where
        P: AsRef<Path>,
    {
        match self.matcher.matched(path, is_directory) {
            Match::None => None,
            Match::Whitelist(_) => None,
            Match::Ignore(glob) => Some(glob),
        }
    }

    /// Returns the glob that matches this `path` or any parent, or `None` if no glob
    /// matches
    ///
    /// Implementation is based on [ignore::gitignore::Gitignore::matched_path_or_any_parents],
    /// excluding the `assert!(!path.has_root())` check since default patterns don't
    /// depend on the `root`.
    pub fn matched_path_or_any_parents<P>(&self, path: P, is_directory: bool) -> Option<&Glob>
    where
        P: AsRef<Path>,
    {
        let mut path = path.as_ref();

        match self.matched(path, is_directory) {
            None => (), // walk up
            a_match => return a_match,
        }
        while let Some(parent) = path.parent() {
            match self.matched(parent, /* is_directory */ true) {
                None => path = parent, // walk up
                a_match => return a_match,
            }
        }

        None
    }
}

#[cfg(test)]
mod test {
    use crate::file_patterns::DefaultFilePatterns;
    use crate::file_patterns::FilePatterns;
    use std::path::Path;

    fn from_str<P: AsRef<Path>>(root: P, pattern: &str) -> FilePatterns {
        let patterns = vec![pattern];
        FilePatterns::try_from_iter(root, patterns).unwrap()
    }

    macro_rules! ignored {
        ($root:expr, $gi:expr, $path:expr) => {
            ignored!($root, $gi, $path, false);
        };
        ($root:expr, $gi:expr, $path:expr, $is_dir:expr) => {
            let ignore = from_str($root, $gi);
            assert!(ignore.matched($path, $is_dir).is_some());
        };
    }

    macro_rules! not_ignored {
        ($root:expr, $gi:expr, $path:expr) => {
            not_ignored!($root, $gi, $path, false);
        };
        ($root:expr, $gi:expr, $path:expr, $is_dir:expr) => {
            let ignore = from_str($root, $gi);
            assert!(ignore.matched($path, $is_dir).is_none());
        };
    }

    // These tests confirm behavior that we expect to get from `Gitignore`
    #[test]
    fn test_expected_gitignore_behavior() {
        // By specifying the root directory, all prefixes are stripped
        // relative to this root directory before applying the glob matchers
        //
        // This means that a user specifies `renv/` in `path/to/root/air.toml` and
        // we strip `path/to/root` from `path/to/root/renv/` before applying the matcher,
        // which is nice.
        let root = Path::new("path/to/root");

        // When specified as `renv`, `ignore` matches both files named `renv` and
        // directories named `renv`. Because there is no preceding `/`, the `renv`
        // folder can appear at any depth.
        let pattern = "renv";
        ignored!(root, pattern, "renv", true);
        ignored!(root, pattern, "subdir/renv", true);
        ignored!(root, pattern, "renv");
        not_ignored!(root, pattern, "renv/activate.R");

        // When specified as `renv/`, ignore only matches directories, which affects
        // `matched(path, is_dir = false)`
        let pattern = "renv/";
        ignored!(root, pattern, "renv", true);
        ignored!(root, pattern, "subdir/renv", true);
        not_ignored!(root, pattern, "renv");
        not_ignored!(root, pattern, "renv/activate.R");

        // Adding a preceding `/` makes it absolute, underneath the root
        let pattern = "/renv/";
        ignored!(root, pattern, "renv", true);
        not_ignored!(root, pattern, "subdir/renv", true);

        // Any files or folders under the `renv/` directory, up to the first `/`,
        // and because there is a `/` in there, `renv/` must appear under the gitignore
        // root directory.
        let pattern = "renv/*";
        not_ignored!(root, pattern, "renv", true);
        ignored!(root, pattern, "renv/", true);
        ignored!(root, pattern, "renv/activate.R");
        not_ignored!(root, pattern, "subdir/renv", true);
        ignored!(root, pattern, "renv/subdir", true);
        not_ignored!(root, pattern, "renv/subdir/activate.R");
        not_ignored!(root, pattern, "renv/subdir/python.py");

        // Any files or folders under the `renv/` directory, at any depth, specified using
        // `**` as the standard unix way of saying "any depth". `renv/` must appear under
        // the gitignore root directory.
        let pattern = "renv/**";
        not_ignored!(root, pattern, "renv", true);
        ignored!(root, pattern, "renv/", true);
        ignored!(root, pattern, "renv/activate.R");
        not_ignored!(root, pattern, "subdir/renv", true);
        ignored!(root, pattern, "renv/subdir", true);
        ignored!(root, pattern, "renv/subdir/activate.R");
        ignored!(root, pattern, "renv/subdir/python.py");

        // Any R files under the `renv/` directory, but stops at `/` due to
        // `literal_separator(true)` being hardcoded by Gitignorebuilder, so doesn't match
        // if R files are inside subdirectories
        let pattern = "renv/*.R";
        ignored!(root, pattern, "renv/activate.R");
        not_ignored!(root, pattern, "foo/renv/activate.R");
        not_ignored!(root, pattern, "renv/subdir/activate.R");

        // Any R files under the `renv/` directory at any depth, specified using
        // the standard Unix glob way of `/**/`.
        let pattern = "renv/**/*.R";
        ignored!(root, pattern, "renv/activate.R");
        not_ignored!(root, pattern, "foo/renv/activate.R");
        ignored!(root, pattern, "renv/subdir/activate.R");

        // Any R files under the `renv/` directory at any depth, and `renv/` itself
        // can also appear anywhere.
        let pattern = "**/renv/**/*.R";
        ignored!(root, pattern, "renv/activate.R");
        ignored!(root, pattern, "foo/renv/activate.R");
        ignored!(root, pattern, "renv/subdir/activate.R");

        // With gitignore, top level `cpp11.R` with no preceding `/` matches everywhere,
        // regardless of depth. This is desired!
        //
        // `literal_separator(true)` is always on (Gitignore hardcodes it), so in theory
        // `cpp11.R` would not cross the `/` boundary. But when there is no `/` present in
        // the line, the builder prefixes with `**/` to mimic the nice git behavior,
        // giving us `**/cpp11.R` in the underlying globset, so even subdirectories match
        // here.
        let pattern = "cpp11.R";
        ignored!(root, pattern, "cpp11.R");
        ignored!(root, pattern, "renv/cpp11.R");

        // Adding a preceding `/` makes it absolute, preventing subdirectories from matching
        let pattern = "/cpp11.R";
        ignored!(root, pattern, "cpp11.R");
        not_ignored!(root, pattern, "renv/cpp11.R");

        // Testing `import-standalone-*.R` in particular because it has a `*`, but
        // otherwise it works the same as `cpp11.R`
        let pattern = "import-standalone-*.R";
        ignored!(root, pattern, "import-standalone-types.R");
        ignored!(root, pattern, "import-standalone-type-check.R");
        ignored!(root, pattern, "R/import-standalone-type-check.R");
    }

    #[test]
    fn test_default_file_pattern_works_with_rooted_paths() -> anyhow::Result<()> {
        let patterns = DefaultFilePatterns::try_from_iter(vec!["**/cpp11.R"])?;

        // These look like they have a `root`, which `Gitignore::matched_path_or_any_parents()`
        // would typically panic on, so we have our own version to avoid this, since all
        // of our default patterns include `**/` and are root agnostic.
        assert!(patterns
            .matched_path_or_any_parents("/etc/cpp11.R", false)
            .is_some());

        assert!(patterns
            .matched_path_or_any_parents("C:/etc/cpp11.R", false)
            .is_some());

        Ok(())
    }
}
