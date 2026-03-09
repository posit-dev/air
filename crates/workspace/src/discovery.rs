//
// discovery.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use ignore::DirEntry;
use ignore::gitignore::Glob;
use rustc_hash::FxHashSet;
use std::path::Path;
use std::path::PathBuf;

use crate::resolve::PathResolver;
use crate::settings::DefaultExcludePatterns;
use crate::settings::DefaultIncludePatterns;
use crate::settings::ExcludePatterns;
use crate::settings::Settings;
use crate::toml::find_air_toml_in_directory;
use crate::toml::parse_air_toml;

#[derive(Debug)]
pub struct DiscoveredSettings {
    pub directory: PathBuf,
    pub settings: Settings,
}

/// This is the core function for walking a set of `paths` looking for `air.toml`s.
///
/// You typically follow this function up by loading the set of returned path into a
/// [crate::resolve::PathResolver].
///
/// For each `path`, we:
/// - Walk up its ancestors, looking for an `air.toml`
/// - TODO(hierarchical): Walk down its children, looking for nested `air.toml`s
pub fn discover_settings<P: AsRef<Path>>(paths: &[P]) -> anyhow::Result<Vec<DiscoveredSettings>> {
    let paths: Vec<PathBuf> = paths.iter().map(fs::normalize_path).collect();

    let mut seen = FxHashSet::default();
    let mut discovered_settings = Vec::with_capacity(paths.len());

    // Discover all `Settings` across all `paths`, looking up each path's directory tree
    for path in &paths {
        for ancestor in path.ancestors() {
            let is_new_ancestor = seen.insert(ancestor);

            if !is_new_ancestor {
                // We already visited this ancestor, we can stop here.
                break;
            }

            if let Some(toml) = find_air_toml_in_directory(ancestor) {
                let settings = parse_settings(&toml, ancestor)?;
                discovered_settings.push(DiscoveredSettings {
                    directory: ancestor.to_path_buf(),
                    settings,
                });
                break;
            }
        }
    }

    for discovered_setting in &discovered_settings {
        tracing::trace!(
            "Discovered settings at '{directory}'",
            directory = discovered_setting.directory.display()
        );
    }

    // TODO(hierarchical): Also iterate into the directories and collect `air.toml`
    // found nested withing the directories for hierarchical support

    Ok(discovered_settings)
}

/// Parse [Settings] from a given `air.toml`
// TODO(hierarchical): Allow for an `extends` option in `air.toml`, which will make things
// more complex, but will be very useful once we support hierarchical configuration as a
// way of "inheriting" most top level configuration while slightly tweaking it in a nested directory.
fn parse_settings(toml: &Path, root_directory: &Path) -> anyhow::Result<Settings> {
    let options = parse_air_toml(toml)?;
    let settings = options.into_settings(root_directory)?;
    Ok(settings)
}

type DiscoveredFiles = Vec<Result<PathBuf, ignore::Error>>;

/// File discovery mode
///
/// Currently only for the formatter, but it is likely we would use the same
/// infrastructure for a linter, but with different `exclude` and `include` rules.
#[derive(Debug)]
pub enum Mode {
    /// Use formatter specific `exclude` and `include` rules
    Format,
}

#[derive(Debug, Copy, Clone)]
pub enum Exclude {
    /// Filter out files matching `exclude` and `default_exclude` patterns
    Matched,

    /// Exclude nothing
    Nothing,
}

#[derive(Debug, Copy, Clone)]
pub enum Include {
    /// Filter for files matching `default_include` patterns
    Matched,

    /// Include everything
    Everything,
}

/// For each provided `path`, recursively search for any R files within that `path`
/// that match our criteria
///
/// NOTE: Make sure that the criteria that guide `path` discovery are also
/// consistently applied to [discover_settings()].
pub fn discover_r_file_paths<P: AsRef<Path>>(
    paths: &[P],
    resolver: &PathResolver<Settings>,
    mode: Mode,
    exclude: Exclude,
    include: Include,
) -> DiscoveredFiles {
    let paths: Vec<PathBuf> = paths.iter().map(fs::normalize_path).collect();

    let Some((first_path, paths)) = paths.split_first() else {
        // No paths provided
        return Vec::new();
    };

    let mut builder = ignore::WalkBuilder::new(first_path);

    for path in paths {
        builder.add(path);
    }

    // TODO: Make these configurable options (possibly just one?)
    // Right now we explicitly call them even though they are `true` by default
    // to remind us to expose them.
    //
    // "This toggles, as a group, all the filters that are enabled by default"
    // builder.standard_filters(true)
    builder.hidden(true);
    builder.parents(true);
    builder.ignore(false);
    builder.git_ignore(true);
    builder.git_global(true);
    builder.git_exclude(true);

    // Prefer `available_parallelism()`, with a max of 12 threads
    builder.threads(
        std::thread::available_parallelism()
            .map_or(1, std::num::NonZeroUsize::get)
            .min(12),
    );

    let walker = builder.build_parallel();

    // Run the `WalkParallel` to collect all R files.
    let state = FilesState::new(resolver, mode, exclude, include);
    let mut visitor_builder = FilesVisitorBuilder::new(&state);
    walker.visit(&mut visitor_builder);

    state.finish()
}

/// Shared state across the threads of the walker
struct FilesState<'resolver> {
    files: std::sync::Mutex<DiscoveredFiles>,
    resolver: &'resolver PathResolver<Settings>,
    mode: Mode,
    exclude: Exclude,
    include: Include,
}

impl<'resolver> FilesState<'resolver> {
    fn new(
        resolver: &'resolver PathResolver<Settings>,
        mode: Mode,
        exclude: Exclude,
        include: Include,
    ) -> Self {
        Self {
            files: std::sync::Mutex::new(Vec::new()),
            resolver,
            mode,
            exclude,
            include,
        }
    }

    fn finish(self) -> DiscoveredFiles {
        self.files.into_inner().unwrap()
    }
}

/// Object capable of building a [FilesVisitor]
///
/// Implements the `build()` method of [ignore::ParallelVisitorBuilder], which
/// [ignore::WalkParallel] utilizes to create one [FilesVisitor] per thread.
struct FilesVisitorBuilder<'state, 'resolver> {
    state: &'state FilesState<'resolver>,
}

impl<'state, 'resolver> FilesVisitorBuilder<'state, 'resolver> {
    fn new(state: &'state FilesState<'resolver>) -> Self {
        Self { state }
    }
}

impl<'state> ignore::ParallelVisitorBuilder<'state> for FilesVisitorBuilder<'state, '_> {
    /// Constructs the per-thread [FilesVisitor], called for us by `ignore`
    fn build(&mut self) -> Box<dyn ignore::ParallelVisitor + 'state> {
        Box::new(FilesVisitor {
            files: vec![],
            state: self.state,
        })
    }
}

/// Object that implements [ignore::ParallelVisitor]'s `visit()` method
///
/// A files visitor has its `visit()` method repeatedly called. It modifies its own
/// synchronous state by pushing to its thread specific `files` while visiting. On `Drop`,
/// the collected `files` are appended to the global set of `state.files`.
struct FilesVisitor<'state, 'resolver> {
    files: DiscoveredFiles,
    state: &'state FilesState<'resolver>,
}

impl ignore::ParallelVisitor for FilesVisitor<'_, '_> {
    /// Visit a file in the tree
    ///
    /// Visiting a file requires two actions:
    /// - Deciding whether or not to accept the file
    /// - Deciding whether or not to `WalkState::Continue` or `WalkState::Skip`
    ///
    /// ## Importance of `WalkState::Skip`
    ///
    /// We only return `WalkState::Skip` when we reject a file due to our `exclude`
    /// criteria, but this case is extremely important. It is a nice optimization because
    /// if we reject `renv/` then we never look at `renv/activate.R` at all, but it also
    /// affects the behavior of `exclude` in general. With `exclude = ["renv/"]`,
    /// `matches("renv")` of course returns `true`, but `matches("renv/activate.R")`
    /// returns `false`. This means that in order to correctly implement the `exclude`
    /// behavior, we absolutely cannot recurse into `renv/` after we reject it, otherwise
    /// we will blindly accept its children unless we run `matches()` on each parent
    /// directory of `"renv/activate.R"` as well, which would be wasteful and expensive.
    fn visit(&mut self, result: std::result::Result<DirEntry, ignore::Error>) -> ignore::WalkState {
        // Determine if `ignore` gave us a valid `result` or not
        let entry = match result {
            Ok(entry) => entry,
            Err(error) => {
                // Store error but continue walking
                self.files.push(Err(error));
                return ignore::WalkState::Continue;
            }
        };

        match self.state.mode {
            Mode::Format => self.visit_format(entry),
        }
    }
}

impl FilesVisitor<'_, '_> {
    /// Visit each entry using formatter specific `exclude` and `include` rules
    fn visit_format(&mut self, entry: DirEntry) -> ignore::WalkState {
        let path = entry.path();

        // An entry is directly supplied if the user provided it directly on the command
        // line, and it was not discovered by looking into a directory
        let is_directly_supplied = entry.depth() == 0;

        let is_directory = entry.file_type().is_none_or(|ft| ft.is_dir());

        // Retrieve the settings for this `path`
        let settings = self.state.resolver.resolve_or_fallback(path);

        match self.state.exclude {
            Exclude::Matched => {
                // Throw out any directly supplied `path` where the path itself or any
                // parent is excluded. For example, `air format cpp11.R` is immediately
                // skipped since `**/cpp11.R` is an excluded pattern. Similarly, `air
                // format renv/activate.R` and `air format renv/subdir/` are immediately
                // skipped since `**/renv/` is an excluded pattern and we look at the
                // directly supplied path's parents.
                if is_directly_supplied
                    && let Some(glob) = any_exclude_matched_path_or_any_parents(
                        path,
                        is_directory,
                        settings.format.exclude.as_ref(),
                        settings.format.default_exclude.as_ref(),
                    )
                {
                    tracing::trace!(
                        "Excluded due to '{glob}': {path}",
                        glob = glob.original(),
                        path = path.display()
                    );
                    return ignore::WalkState::Skip;
                }

                // If this `path` was found from recursive search, don't check the path's
                // parents when looking at exclusion rules
                if !is_directly_supplied
                    && let Some(glob) = any_exclude_matched_path(
                        path,
                        is_directory,
                        settings.format.exclude.as_ref(),
                        settings.format.default_exclude.as_ref(),
                    )
                {
                    tracing::trace!(
                        "Excluded due to '{glob}': {path}",
                        glob = glob.original(),
                        path = path.display()
                    );
                    return ignore::WalkState::Skip;
                }
            }
            Exclude::Nothing => {
                // Exclusion patterns are not considered
            }
        }

        if is_directory {
            // Recurse into any directory that hasn't been excluded
            return ignore::WalkState::Continue;
        }

        // Now handle files
        match self.state.include {
            Include::Matched => {
                // Files that haven't been excluded are only included if they match a
                // `default_include` pattern, even if they are directly supplied by the user!
                match any_include_matched_path(path, settings.format.default_include.as_ref()) {
                    Some(glob) => {
                        tracing::trace!(
                            "Included due to '{glob}': {path}",
                            glob = glob.original(),
                            path = path.display()
                        );
                        self.files.push(Ok(entry.into_path()));
                        ignore::WalkState::Continue
                    }
                    None => {
                        tracing::trace!(
                            "Excluded due to not matching an include: {path}",
                            path = path.display()
                        );
                        ignore::WalkState::Continue
                    }
                }
            }
            Include::Everything => {
                tracing::trace!(
                    "Included due to including everything: {path}",
                    path = path.display()
                );
                self.files.push(Ok(entry.into_path()));
                ignore::WalkState::Continue
            }
        }
    }
}

impl Drop for FilesVisitor<'_, '_> {
    fn drop(&mut self) {
        // Lock the global shared set of `files`
        // Unwrap: If we can't lock the mutex then something is very wrong
        let mut files = self.state.files.lock().unwrap();

        // Transfer files gathered on this thread to the global set
        if files.is_empty() {
            *files = std::mem::take(&mut self.files);
        } else {
            files.append(&mut self.files);
        }
    }
}

/// Returns the glob that matches this `path`, or `None` if no glob matches
///
/// Does not search parents, so a path of `renv/activate.R` would not match
/// a pattern of `**/renv/`
fn any_exclude_matched_path<'patterns, P: AsRef<Path>>(
    path: P,
    is_directory: bool,
    exclude: Option<&'patterns ExcludePatterns>,
    default_exclude: Option<&'patterns DefaultExcludePatterns>,
) -> Option<&'patterns Glob> {
    let path = path.as_ref();

    if let Some(glob) = exclude.and_then(|exclude| exclude.matched(path, is_directory)) {
        return Some(glob);
    }

    if let Some(glob) =
        default_exclude.and_then(|default_exclude| default_exclude.matched(path, is_directory))
    {
        return Some(glob);
    }

    None
}

/// Returns the glob that matches this `path`, or `None` if no glob matches
///
/// Searches parents, so a path of `renv/activate.R` would match a pattern of `**/renv/`,
/// but this has a performance cost, so should only be used when necessary.
pub fn any_exclude_matched_path_or_any_parents<'patterns, P: AsRef<Path>>(
    path: P,
    is_directory: bool,
    exclude: Option<&'patterns ExcludePatterns>,
    default_exclude: Option<&'patterns DefaultExcludePatterns>,
) -> Option<&'patterns Glob> {
    let path = path.as_ref();

    if let Some(glob) =
        exclude.and_then(|exclude| exclude.matched_path_or_any_parents(path, is_directory))
    {
        return Some(glob);
    }

    if let Some(glob) = default_exclude
        .and_then(|default_exclude| default_exclude.matched_path_or_any_parents(path, is_directory))
    {
        return Some(glob);
    }

    None
}

/// Returns the glob that matches this `path`, or `None` if no glob matches
///
/// Includes are only about files, so this is only ever called on a file and never a
/// directory
pub fn any_include_matched_path<P: AsRef<Path>>(
    path: P,
    default_include: Option<&DefaultIncludePatterns>,
) -> Option<&Glob> {
    const IS_DIRECTORY: bool = false;

    let path = path.as_ref();

    default_include.and_then(|default_include| default_include.matched(path, IS_DIRECTORY))
}

#[cfg(test)]
mod test {
    use anyhow::Context;
    use tempfile::TempDir;

    use crate::discovery::Exclude;
    use crate::discovery::Include;
    use crate::discovery::Mode;
    use crate::discovery::discover_r_file_paths;
    use crate::discovery::discover_settings;
    use crate::resolve::PathResolver;
    use crate::settings::Settings;

    #[test]
    fn test_finds_typical_r_files() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        std::fs::create_dir(tempdir.join("R"))?;
        std::fs::create_dir(tempdir.join("tests"))?;
        std::fs::create_dir(tempdir.join("tests").join("testthat"))?;

        let test_path = tempdir.join("R").join("test.R");
        std::fs::write(&test_path, b"")?;

        let test2_path = tempdir.join("tests").join("testthat").join("test2.R");
        std::fs::write(&test2_path, b"")?;

        let resolver = PathResolver::new(Settings::default());

        let mut paths = discover_r_file_paths(
            &[tempdir],
            &resolver,
            Mode::Format,
            Exclude::Matched,
            Include::Matched,
        );

        assert_eq!(paths.len(), 2);
        let mut paths = [paths.pop().unwrap()?, paths.pop().unwrap()?];
        paths.sort();

        let mut expect = [test_path, test2_path];
        expect.sort();

        assert_eq!(paths, expect);

        Ok(())
    }

    #[test]
    fn test_default_includes_respected_during_recursive_discovery() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        let test_r = tempdir.join("test.R");
        let test_py = tempdir.join("test.py");

        std::fs::write(&test_r, b"")?;
        std::fs::write(&test_py, b"")?;

        let resolver = PathResolver::new(Settings::default());

        let mut paths = discover_r_file_paths(
            &[tempdir],
            &resolver,
            Mode::Format,
            Exclude::Matched,
            Include::Matched,
        );

        assert_eq!(paths.len(), 1);
        let path = paths.pop().unwrap()?;

        assert_eq!(path, test_r);

        Ok(())
    }

    #[test]
    fn test_default_includes_respected_for_directly_supplied_paths() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        let test_big_r = tempdir.join("test1.R");
        let test_little_r = tempdir.join("test2.r");
        let test_qmd = tempdir.join("test3.qmd");

        std::fs::write(&test_big_r, b"")?;
        std::fs::write(&test_little_r, b"")?;
        std::fs::write(&test_qmd, b"")?;

        {
            // `{tempdir}/test1.R`
            // Part of default includes, so accepted
            let start = &[&test_big_r];
            let resolver = PathResolver::new(Settings::default());
            let mut paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert_eq!(paths.len(), 1);
            assert_eq!(paths.pop().unwrap().unwrap(), test_big_r);
        }

        {
            // `{tempdir}/test2.r`
            // Part of default includes, so accepted
            let start = &[&test_little_r];
            let resolver = PathResolver::new(Settings::default());
            let mut paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert_eq!(paths.len(), 1);
            assert_eq!(paths.pop().unwrap().unwrap(), test_little_r);
        }

        {
            // `{tempdir}/test3.qmd`
            // Not part of default includes, so rejected even though the user supplied it
            // directly. Historically this has proven to be important, because people will
            // do `air format my.qmd` and be surprised if by chance it parses then happens
            // to bork their qmd.
            let start = &[&test_qmd];
            let resolver = PathResolver::new(Settings::default());
            let paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert_eq!(paths.len(), 0);
        }

        Ok(())
    }

    #[test]
    fn test_default_exclude_patterns() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        std::fs::create_dir(tempdir.join("R"))?;
        std::fs::create_dir(tempdir.join("renv"))?;
        std::fs::create_dir(tempdir.join("renv").join("subdir"))?;
        std::fs::create_dir(tempdir.join("revdep"))?;
        std::fs::create_dir(tempdir.join("revdep").join("pkg"))?;

        // Find this one
        let test_path = tempdir.join("R").join("test.R");
        std::fs::write(&test_path, b"")?;

        // Exclude all of these
        std::fs::write(tempdir.join("renv").join("activate.R"), b"")?;
        std::fs::write(tempdir.join("revdep").join("pkg").join("foo.R"), b"")?;
        std::fs::write(tempdir.join("R").join("cpp11.R"), b"")?;
        std::fs::write(tempdir.join("R").join("RcppExports.R"), b"")?;
        std::fs::write(tempdir.join("R").join("extendr-wrappers.R"), b"")?;
        std::fs::write(tempdir.join("R").join("import-standalone-types.R"), b"")?;

        {
            // `{tempdir}`
            // Folder containing folders and files that match default excludes
            let start = &[tempdir];
            let resolver = PathResolver::new(Settings::default());
            let mut paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert_eq!(paths.len(), 1);
            assert_eq!(paths.pop().unwrap().unwrap(), test_path);
        }

        {
            // `{tempdir}/R/cpp11.R`
            // Directly supplied file matching default excludes is excluded
            // https://github.com/posit-dev/air/issues/472
            let start = &[tempdir.join("R").join("cpp11.R")];
            let resolver = PathResolver::new(Settings::default());
            let paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert_eq!(paths.len(), 0);
        }

        {
            // `{tempdir}/renv`
            // Directly supplied directory matching default excludes is excluded
            // https://github.com/posit-dev/air/issues/472
            let start = &[tempdir.join("renv")];
            let resolver = PathResolver::new(Settings::default());
            let paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert_eq!(paths.len(), 0);
        }

        {
            // `{tempdir}/renv/activate.R`
            // Directly supplied file with parent matching default excludes is excluded
            // https://github.com/posit-dev/air/issues/472
            let start = &[tempdir.join("renv").join("activate.R")];
            let resolver = PathResolver::new(Settings::default());
            let paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert_eq!(paths.len(), 0);
        }

        {
            // `{tempdir}/renv/subdir/`
            // Directly supplied directory with parent matching default excludes is excluded
            // https://github.com/posit-dev/air/issues/472
            let start = &[tempdir.join("renv").join("subdir")];
            let resolver = PathResolver::new(Settings::default());
            let paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert_eq!(paths.len(), 0);
        }

        Ok(())
    }

    #[test]
    fn test_user_exclude_patterns() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        let air_path = tempdir.join("air.toml");
        let air_contents = r#"
[format]
exclude = ["exclude/"]
"#;
        std::fs::write(&air_path, air_contents)?;

        std::fs::create_dir(tempdir.join("exclude"))?;
        std::fs::create_dir(tempdir.join("exclude").join("subdir"))?;

        // Should always exclude all of these
        std::fs::write(tempdir.join("exclude").join("test.R"), b"")?;
        std::fs::write(tempdir.join("exclude").join("subdir").join("test.R"), b"")?;

        {
            // `{tempdir}`
            let start = &[tempdir];

            let mut settings = discover_settings(start)?;
            let settings = settings.pop().context("Should find air.toml")?;

            let mut resolver = PathResolver::new(Settings::default());
            resolver.add(&settings.directory, settings.settings);

            let paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert!(paths.is_empty());
        }

        {
            // `{tempdir}/exclude/`
            // Directly supplied directory matching user exclude is excluded
            // https://github.com/posit-dev/air/issues/472
            let start = &[tempdir.join("exclude")];

            let mut settings = discover_settings(start)?;
            let settings = settings.pop().context("Should find air.toml")?;

            let mut resolver = PathResolver::new(Settings::default());
            resolver.add(&settings.directory, settings.settings);

            let paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert!(paths.is_empty());
        }

        {
            // `{tempdir}/exclude/test.R`
            // Directly supplied file with parent matching user exclude is excluded
            // https://github.com/posit-dev/air/issues/472
            let start = &[tempdir.join("exclude").join("test.R")];

            let mut settings = discover_settings(start)?;
            let settings = settings.pop().context("Should find air.toml")?;

            let mut resolver = PathResolver::new(Settings::default());
            resolver.add(&settings.directory, settings.settings);

            let paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert!(paths.is_empty());
        }

        {
            // `{tempdir}/exclude/subdir/`
            // Directly supplied folder with parent matching user exclude is excluded
            // https://github.com/posit-dev/air/issues/472
            let start = &[tempdir.join("exclude").join("subdir")];

            let mut settings = discover_settings(start)?;
            let settings = settings.pop().context("Should find air.toml")?;

            let mut resolver = PathResolver::new(Settings::default());
            resolver.add(&settings.directory, settings.settings);

            let paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert!(paths.is_empty());
        }

        Ok(())
    }

    #[test]
    fn test_exclude_nothing_allows_default_excluded_files() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        std::fs::create_dir(tempdir.join("R"))?;

        let cpp11_path = tempdir.join("R").join("cpp11.R");
        std::fs::write(&cpp11_path, b"")?;

        // `cpp11.R` is a default exclude, but `Exclude::Nothing` bypasses it
        let start = &[&cpp11_path];
        let resolver = PathResolver::new(Settings::default());
        let mut paths = discover_r_file_paths(
            start,
            &resolver,
            Mode::Format,
            Exclude::Nothing,
            Include::Matched,
        );
        assert_eq!(paths.len(), 1);
        assert_eq!(paths.pop().unwrap().unwrap(), cpp11_path);

        Ok(())
    }

    #[test]
    fn test_exclude_nothing_recursive_case() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        std::fs::create_dir(tempdir.join("R"))?;
        std::fs::create_dir(tempdir.join("vignettes"))?;

        let test_path = tempdir.join("R").join("test.R");
        std::fs::write(&test_path, b"")?;

        let cpp11_path = tempdir.join("R").join("cpp11.R");
        std::fs::write(&cpp11_path, b"")?;

        let test_qmd = tempdir.join("vignettes").join("test.qmd");
        std::fs::write(&test_qmd, b"")?;

        // Recursing into `tempdir` with `Exclude::Nothing` discovers `test.R` and
        // `cpp11.R`, but not `test.qmd`
        let start = &[tempdir];
        let resolver = PathResolver::new(Settings::default());
        let mut paths = discover_r_file_paths(
            start,
            &resolver,
            Mode::Format,
            Exclude::Nothing,
            Include::Matched,
        );

        assert_eq!(paths.len(), 2);
        let mut paths = [paths.pop().unwrap().unwrap(), paths.pop().unwrap().unwrap()];
        paths.sort();
        let mut expect = [test_path, cpp11_path];
        expect.sort();
        assert_eq!(paths, expect);

        Ok(())
    }

    #[test]
    fn test_include_everything_allows_non_r_files() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        let test_qmd = tempdir.join("test.qmd");
        std::fs::write(&test_qmd, b"")?;

        // `.qmd` is not part of default includes, but `Include::Everything` bypasses that
        let start = &[&test_qmd];
        let resolver = PathResolver::new(Settings::default());
        let mut paths = discover_r_file_paths(
            start,
            &resolver,
            Mode::Format,
            Exclude::Matched,
            Include::Everything,
        );
        assert_eq!(paths.len(), 1);
        assert_eq!(paths.pop().unwrap().unwrap(), test_qmd);

        Ok(())
    }

    #[test]
    fn test_include_everything_recursive_case() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        std::fs::create_dir(tempdir.join("R"))?;
        std::fs::create_dir(tempdir.join("vignettes"))?;

        let test_path = tempdir.join("R").join("test.R");
        std::fs::write(&test_path, b"")?;

        let cpp11_path = tempdir.join("R").join("cpp11.R");
        std::fs::write(&cpp11_path, b"")?;

        let test_qmd = tempdir.join("vignettes").join("test.qmd");
        std::fs::write(&test_qmd, b"")?;

        // Recursing into `tempdir` with `Include::Everything` discovers `test.R` and
        // `test.qmd`, but not `cpp11.R`
        let start = &[tempdir];
        let resolver = PathResolver::new(Settings::default());
        let mut paths = discover_r_file_paths(
            start,
            &resolver,
            Mode::Format,
            Exclude::Matched,
            Include::Everything,
        );

        assert_eq!(paths.len(), 2);
        let mut paths = [paths.pop().unwrap().unwrap(), paths.pop().unwrap().unwrap()];
        paths.sort();
        let mut expect = [test_path, test_qmd];
        expect.sort();
        assert_eq!(paths, expect);

        Ok(())
    }

    #[test]
    fn test_gitignore() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        // You have to create a `.git` directory for `ignore` to respect `.gitignore` by default
        std::fs::create_dir(tempdir.join(".git"))?;

        let gitignore_path = tempdir.join(".gitignore");
        let gitignore_contents = r#"
ignore/
"#;
        std::fs::write(&gitignore_path, gitignore_contents)?;

        std::fs::create_dir(tempdir.join("ignore"))?;
        std::fs::create_dir(tempdir.join("ignore").join("subdir"))?;

        std::fs::write(tempdir.join("ignore").join("test.R"), b"")?;
        std::fs::write(tempdir.join("ignore").join("subdir").join("test.R"), b"")?;

        {
            // `{tempdir}/`
            // When `ignore/` is "discovered" via recursive search, the `.gitignore` is respected
            let start = &[tempdir];
            let resolver = PathResolver::new(Settings::default());
            let paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert!(paths.is_empty());
        }

        {
            // `{tempdir}/ignore/`
            // When `ignore/` is directly provided, `.gitignore` is bypassed unlike our
            // `exclude` behavior that searches the parents of any user provided
            // directories. This is {ignore} specific behavior that we don't control, and
            // {ignore}'s author prefers this. It's probably not a very big deal, because
            // it's unlikely that a user (or pre-commit or RStudio) will call `air format
            // <folder-that-has-been-gitignored>` directly.
            let start = &[tempdir.join("ignore")];
            let resolver = PathResolver::new(Settings::default());
            let paths = discover_r_file_paths(
                start,
                &resolver,
                Mode::Format,
                Exclude::Matched,
                Include::Matched,
            );
            assert_eq!(paths.len(), 2);
        }

        Ok(())
    }
}
