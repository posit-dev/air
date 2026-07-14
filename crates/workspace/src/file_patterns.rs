use std::path::Path;
use std::path::PathBuf;

use ignore::Match;
use ignore::gitignore::Gitignore;
use ignore::gitignore::GitignoreBuilder;
use ignore::gitignore::Glob;

/// Matcher for globs rooted at a directory
///
/// When constructing the matcher, you supply a `root` path along with the `patterns` to
/// be included in the matcher. When [RootedFilePatterns::matched] is called, the `root`
/// path is always stripped from `path` before matching is done. This ensures that users
/// can specify `/special.R` in their `air.toml` to match only `{root}/special.R`, and not
/// also `{root}/subdir/special.R`.
#[derive(Clone, Debug)]
pub struct RootedFilePatterns {
    matcher: Gitignore,
}

/// Matcher for globs with no `root`, matching at any depth
///
/// Compared to [RootedFilePatterns], [UnrootedFilePatterns] is special because it does
/// not allow specification of a `root` path. This is what backs default includes and
/// excludes, which apply with or without a physical `air.toml`, as well as user level
/// `air.toml` `exclude` patterns, which have no project directory to root against. To
/// ensure this works correctly, we have to make two main changes:
///
/// - Every `pattern` must match at any depth rather than relative to a `root`, which is
///   enforced by [is_unrooted] in [UnrootedFilePatterns::try_from_iter] at creation time.
///
/// - [UnrootedFilePatterns::matched_path_or_any_parents] is custom rather than deferring
///   to [Gitignore]. [Gitignore]'s version panics if `path` still has a root component
///   after `root` stripping (a leading `/` on Unix, or `C:/` on Windows). We never strip
///   a `root` and our globs don't depend on one, so that restriction doesn't apply.
#[derive(Clone, Debug)]
pub struct UnrootedFilePatterns {
    matcher: Gitignore,
}

impl RootedFilePatterns {
    /// Construct [RootedFilePatterns] from an iterator of patterns
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
    /// More expensive than [RootedFilePatterns::matched], but is required in the LSP where you
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

impl UnrootedFilePatterns {
    /// Construct [UnrootedFilePatterns] from an iterator of patterns
    ///
    /// Every pattern must be unrooted, as verified by [is_unrooted]. A rooted pattern is
    /// a hard error, since there is no `root` for it to resolve against.
    pub(crate) fn try_from_iter<'str, I>(patterns: I) -> anyhow::Result<Self>
    where
        I: IntoIterator<Item = &'str str>,
    {
        // Use an empty `root` so that nothing is stripped from a `path` before matching
        let root = PathBuf::new();

        let mut builder = GitignoreBuilder::new(root);

        for pattern in patterns {
            if !is_unrooted(pattern) {
                return Err(err_rooted(pattern));
            }
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
    /// excluding the `assert!(!path.has_root())` check since unrooted patterns don't
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

/// Check if a `pattern` is unrooted
///
/// Unrooted patterns start with `**/` and match at any depth. Additionally, for
/// convenience, simple patterns such as `foo.R` and `folder/` are internally treated as
/// their explicitly unrooted equivalents of `**/foo.R` and `**/folder`.
///
/// Rooted patterns are ones with a leading or interior `/`, such as `/foo.R` and
/// `R/foo.R`, and imply `{root}/foo.R` and `{root}/R/foo.R`.
///
/// Constructed by carefully analyzing [GitignoreBuilder]'s `add_line()`, which itself
/// is derived from git's man page. It should be very stable.
///
/// Ideally we'd somehow analyze the fields in the [GitignoreBuilder] output rather than
/// recreating the rules here, but there is no public API for determining if a supplied
/// pattern was rooted or not (it's admittedly a slightly odd use case from a git
/// perspective).
fn is_unrooted(pattern: &str) -> bool {
    // Strip leading `!`
    let pattern = pattern.strip_prefix('!').unwrap_or(pattern);

    // Explicitly unrooted
    if pattern.starts_with("**/") {
        return true;
    }

    // Implicitly unrooted
    // - `foo.R`
    // - `folder/`
    // but not:
    // - `/foo.R`
    // - `R/foo.R`
    // - `R/**/foo.R`
    let pattern = pattern.strip_suffix('/').unwrap_or(pattern);
    !pattern.contains('/')
}

fn err_rooted(pattern: &str) -> anyhow::Error {
    let rooted_pattern = if pattern.starts_with('/') {
        format!("{{root}}{pattern}")
    } else {
        format!("{{root}}/{pattern}")
    };

    anyhow::anyhow!(
        "Pattern `{pattern}` must be unrooted. It is currently a rooted pattern implying `{rooted_pattern}`.
- Unrooted patterns start with `**/` and match at any depth. For convenience, simple patterns like `foo.R` and `folder/` are also considered to be unrooted.
- Rooted patterns contain a leading or interior `/`, like `/foo.R` or `R/foo.R`, and imply `{{root}}/foo.R` or `{{root}}/R/foo.R`, which can't be resolved by a user level `air.toml`."
    )
}

#[cfg(test)]
mod test {
    use crate::file_patterns::RootedFilePatterns;
    use crate::file_patterns::UnrootedFilePatterns;
    use crate::file_patterns::is_unrooted;
    use std::path::Path;

    fn from_str<P: AsRef<Path>>(root: P, pattern: &str) -> RootedFilePatterns {
        let patterns = vec![pattern];
        RootedFilePatterns::try_from_iter(root, patterns).unwrap()
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
    fn test_unrooted_file_pattern_works_with_rooted_paths() -> anyhow::Result<()> {
        let patterns = UnrootedFilePatterns::try_from_iter(vec!["**/cpp11.R"])?;

        // These look like they have a `root`, which `Gitignore::matched_path_or_any_parents()`
        // would typically panic on, so we have our own version to avoid this, since
        // unrooted patterns match at any depth and don't depend on the `root`.
        assert!(
            patterns
                .matched_path_or_any_parents("/etc/cpp11.R", false)
                .is_some()
        );

        assert!(
            patterns
                .matched_path_or_any_parents("C:/etc/cpp11.R", false)
                .is_some()
        );

        // The `folder/` shorthand also floats to any depth, even though the user didn't
        // write `**/`, because `add_line()` supplies the `**/` prefix for us
        let patterns = UnrootedFilePatterns::try_from_iter(vec!["renv/"])?;
        assert!(
            patterns
                .matched_path_or_any_parents("/etc/renv", true)
                .is_some()
        );

        Ok(())
    }

    #[test]
    fn test_unrooted_file_pattern_rejects_rooted_patterns() {
        // A rooted pattern is a hard error, since there is no `root` for it to resolve against
        let error = UnrootedFilePatterns::try_from_iter(vec!["/foo.R"]).unwrap_err();
        insta::assert_snapshot!(error);
        let error = UnrootedFilePatterns::try_from_iter(vec!["foo/bar.R"]).unwrap_err();
        insta::assert_snapshot!(error);
    }

    #[test]
    fn test_is_unrooted() {
        // An explicit `**/` prefix floats to any depth
        assert!(is_unrooted("**/foo.R"));
        assert!(is_unrooted("**/customfolder/"));
        assert!(is_unrooted("**/renv/**/*.R"));

        // No leading or interior `/` means `add_line()` supplies the `**/` prefix,
        // so these float to any depth. A trailing `/` (directory-only) is still fine.
        assert!(is_unrooted("foo.R"));
        assert!(is_unrooted("customfolder/"));
        assert!(is_unrooted("import-standalone-*.R"));

        // A leading `!` (whitelist) doesn't affect whether the pattern is rooted
        assert!(is_unrooted("!foo.R"));
        assert!(is_unrooted("!**/foo.R"));

        // A leading or interior `/` roots the pattern at a `root` we don't have
        assert!(!is_unrooted("/foo.R"));
        assert!(!is_unrooted("foo/bar.R"));
        assert!(!is_unrooted("foo/**/bar.R"));
        assert!(!is_unrooted("/customfolder/"));
        assert!(!is_unrooted("!foo/bar.R"));
        assert!(!is_unrooted("/**/foo"));
    }
}
