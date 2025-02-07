//
// discovery.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use ignore::DirEntry;
use rustc_hash::FxHashSet;
use std::path::Path;
use std::path::PathBuf;

use crate::resolve::PathResolver;
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

/// For each provided `path`, recursively search for any R files within that `path`
/// that match our inclusion criteria
///
/// NOTE: Make sure that the inclusion criteria that guide `path` discovery are also
/// consistently applied to [discover_settings()].
pub fn discover_r_file_paths<P: AsRef<Path>>(
    paths: &[P],
    resolver: &PathResolver<Settings>,
    use_format_settings: bool,
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
    let state = FilesState::new(resolver, use_format_settings);
    let mut visitor_builder = FilesVisitorBuilder::new(&state);
    walker.visit(&mut visitor_builder);

    state.finish()
}

/// Shared state across the threads of the walker
struct FilesState<'resolver> {
    files: std::sync::Mutex<DiscoveredFiles>,
    resolver: &'resolver PathResolver<Settings>,
    use_format_settings: bool,
}

impl<'resolver> FilesState<'resolver> {
    fn new(resolver: &'resolver PathResolver<Settings>, use_format_settings: bool) -> Self {
        Self {
            files: std::sync::Mutex::new(Vec::new()),
            resolver,
            use_format_settings,
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
    /// We only return `WalkState::Skip` when we reject a file due to our `ignore`
    /// criteria, but this case is extremely important. It is a nice optimization because
    /// if we reject `renv/` then we never look at `renv/activate.R` at all, but it also
    /// affects the behavior of `ignore` in general. With `ignore = ["renv/"]`,
    /// `matches("renv")` of course returns `true`, but `matches("renv/activate.R")`
    /// returns `false`. This means that in order to correctly implement the `ignore`
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

        let path = entry.path();

        // An entry is explicit if it was provided directly, not discovered by looking into a directory
        let is_explicit = entry.depth() == 0;
        let is_directory = entry.file_type().map_or(true, |ft| ft.is_dir());

        if is_explicit && !is_directory {
            // Accept explicitly provided files, regardless of exclusion/inclusion
            // criteria (including extension). This is the user supplying `air format
            // file.R`. Note we don't do this for directories, i.e. `air format renv`
            // should do nothing since we have a default `ignore` for `renv/`.
            tracing::trace!("Included file due to explicit provision {path:?}");
            self.files.push(Ok(entry.into_path()));
            return ignore::WalkState::Continue;
        }

        // Retrieve the settings for this `path`
        let settings = self.state.resolver.resolve_or_fallback(path);

        if self.is_ignored(path, is_directory, settings) {
            // Skip this file, and if it is a directory skip all of its children!
            return ignore::WalkState::Skip;
        }

        if self.is_included(path, is_directory, settings) {
            // Accept this file
            self.files.push(Ok(entry.into_path()));
            return ignore::WalkState::Continue;
        }

        // Didn't accept this file, just keep going
        tracing::trace!("Excluded file due to fallthrough {path:?}");
        ignore::WalkState::Continue
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

impl FilesVisitor<'_, '_> {
    fn is_ignored(&self, path: &Path, is_directory: bool, settings: &Settings) -> bool {
        // Consult the format specific patterns if we are in a format context
        if self.state.use_format_settings {
            if let Some(glob) = settings.format.ignore.matched(path, is_directory) {
                tracing::trace!(
                    "Ignored file due to '{glob}' in `format.ignore` {path:?}",
                    glob = glob.original()
                );
                return true;
            }
        }

        false
    }

    fn is_included(&self, path: &Path, is_directory: bool, settings: &Settings) -> bool {
        // Consult the format specific patterns if we are in a format context
        if self.state.use_format_settings {
            if let Some(glob) = settings.format.include.matched(path, is_directory) {
                tracing::trace!(
                    "Included file due to '{glob}' in `format.include` {path:?}",
                    glob = glob.original()
                );
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod test {
    use anyhow::Context;
    use tempfile::TempDir;

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

        let mut paths = discover_r_file_paths(&[tempdir], &resolver, true);

        assert_eq!(paths.len(), 2);
        let mut paths = [paths.pop().unwrap()?, paths.pop().unwrap()?];
        paths.sort();

        let mut expect = [test_path, test2_path];
        expect.sort();

        assert_eq!(paths, expect);

        Ok(())
    }

    #[test]
    fn test_default_ignore_patterns() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        std::fs::create_dir(tempdir.join("R"))?;
        std::fs::create_dir(tempdir.join("renv"))?;
        std::fs::create_dir(tempdir.join("revdep"))?;
        std::fs::create_dir(tempdir.join("revdep").join("pkg"))?;

        // Find this one
        let test_path = tempdir.join("R").join("test.R");
        std::fs::write(&test_path, b"")?;

        // Ignore all of these
        std::fs::write(tempdir.join("renv").join("activate.R"), b"")?;
        std::fs::write(tempdir.join("revdep").join("pkg").join("foo.R"), b"")?;
        std::fs::write(tempdir.join("R").join("cpp11.R"), b"")?;
        std::fs::write(tempdir.join("R").join("RcppExports.R"), b"")?;
        std::fs::write(tempdir.join("R").join("extendr-wrappers.R"), b"")?;
        std::fs::write(tempdir.join("R").join("import-standalone-types.R"), b"")?;

        let resolver = PathResolver::new(Settings::default());

        let mut paths = discover_r_file_paths(&[tempdir], &resolver, true);

        assert_eq!(paths.len(), 1);
        assert_eq!(
            paths.pop().context("Should have a path")?.unwrap(),
            test_path
        );

        Ok(())
    }

    #[test]
    fn test_ignores_directory_children() -> anyhow::Result<()> {
        let tempdir = TempDir::new()?;
        let tempdir = tempdir.path();

        let air_path = tempdir.join("air.toml");
        let air_contents = r#"
[format]
ignore = ["ignore/"]
"#;
        std::fs::write(&air_path, air_contents)?;

        std::fs::create_dir(tempdir.join("R"))?;
        std::fs::create_dir(tempdir.join("ignore"))?;
        std::fs::create_dir(tempdir.join("ignore").join("subdir"))?;

        // Ignore all of these
        std::fs::write(tempdir.join("ignore").join("test.R"), b"")?;
        std::fs::write(tempdir.join("ignore").join("subdir").join("test.R"), b"")?;

        let mut resolver = PathResolver::new(Settings::default());

        let mut settings = discover_settings(&[tempdir])?;
        let settings = settings.pop().context("Should find air.toml")?;
        resolver.add(&settings.directory, settings.settings);

        let paths = discover_r_file_paths(&[tempdir], &resolver, true);
        assert!(paths.is_empty());

        Ok(())
    }
}
