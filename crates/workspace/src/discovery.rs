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
use thiserror::Error;

use crate::settings::Settings;
use crate::toml::find_air_toml_in_directory;
use crate::toml::parse_air_toml;
use crate::toml::ParseTomlError;

#[derive(Debug, Error)]
pub enum DiscoverSettingsError {
    #[error(transparent)]
    ParseToml(#[from] ParseTomlError),
}

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
pub fn discover_settings<P: AsRef<Path>>(
    paths: &[P],
) -> Result<Vec<DiscoveredSettings>, DiscoverSettingsError> {
    let paths: Vec<PathBuf> = paths.iter().map(fs::normalize_path).collect();

    let mut seen = FxHashSet::default();
    let mut discovered_settings = Vec::with_capacity(paths.len());

    // Load the `resolver` with `Settings` associated with each `path`
    for path in &paths {
        for ancestor in path.ancestors() {
            let is_new_ancestor = seen.insert(ancestor);

            if !is_new_ancestor {
                // We already visited this ancestor, we can stop here.
                break;
            }

            if let Some(toml) = find_air_toml_in_directory(ancestor) {
                let settings = parse_settings(&toml)?;
                discovered_settings.push(DiscoveredSettings {
                    directory: ancestor.to_path_buf(),
                    settings,
                });
                break;
            }
        }
    }

    // TODO(hierarchical): Also iterate through the directories and collect `air.toml`
    // found nested withing the directories for hierarchical support

    Ok(discovered_settings)
}

/// Parse [Settings] from a given `air.toml`
// TODO(hierarchical): Allow for an `extends` option in `air.toml`, which will make things
// more complex, but will be very useful once we support hierarchical configuration as a
// way of "inheriting" most top level configuration while slightly tweaking it in a nested directory.
fn parse_settings(toml: &Path) -> Result<Settings, ParseTomlError> {
    let options = parse_air_toml(toml)?;
    let settings = options.into_settings();
    Ok(settings)
}

type DiscoveredFiles = Vec<Result<PathBuf, ignore::Error>>;

/// For each provided `path`, recursively search for any R files within that `path`
/// that match our inclusion criteria
///
/// NOTE: Make sure that the inclusion criteria that guide `path` discovery are also
/// consistently applied to [discover_settings()].
pub fn discover_r_file_paths<P: AsRef<Path>>(paths: &[P]) -> DiscoveredFiles {
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
    let state = FilesState::new();
    let mut visitor_builder = FilesVisitorBuilder::new(&state);
    walker.visit(&mut visitor_builder);

    state.finish()
}

/// Shared state across the threads of the walker
struct FilesState {
    files: std::sync::Mutex<DiscoveredFiles>,
}

impl FilesState {
    fn new() -> Self {
        Self {
            files: std::sync::Mutex::new(Vec::new()),
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
struct FilesVisitorBuilder<'state> {
    state: &'state FilesState,
}

impl<'state> FilesVisitorBuilder<'state> {
    fn new(state: &'state FilesState) -> Self {
        Self { state }
    }
}

impl<'state> ignore::ParallelVisitorBuilder<'state> for FilesVisitorBuilder<'state> {
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
///
/// The files visitor can influence the walk process by returning an [ignore::WalkState].
/// In particular, returning [ignore::WalkState::Skip] when the current entry is a
/// directory will prevent the walker from recursing into that directory.
struct FilesVisitor<'state> {
    files: DiscoveredFiles,
    state: &'state FilesState,
}

impl ignore::ParallelVisitor for FilesVisitor<'_> {
    fn visit(&mut self, result: std::result::Result<DirEntry, ignore::Error>) -> ignore::WalkState {
        // TODO: Add in `WalkState::Skip` behavior when we have `exclude` support

        let entry = match result {
            Ok(entry) => entry,
            Err(error) => {
                // Store error but continue walking
                self.files.push(Err(error));
                return ignore::WalkState::Continue;
            }
        };

        if let Some(file) = as_match(entry) {
            self.files.push(Ok(file));
        }

        ignore::WalkState::Continue
    }
}

impl Drop for FilesVisitor<'_> {
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

// Decide whether or not to accept an `entry` based on include/exclude rules.
fn as_match(entry: DirEntry) -> Option<PathBuf> {
    // Ignore directories
    if entry.file_type().map_or(true, |ft| ft.is_dir()) {
        return None;
    }

    // Accept all files that are passed-in directly, even non-R files
    if entry.depth() == 0 {
        let path = entry.into_path();
        return Some(path);
    }

    // Otherwise check if we should accept this entry
    // TODO: Many other checks based on user exclude/includes
    let path = entry.into_path();

    if !fs::has_r_extension(&path) {
        return None;
    }

    Some(path)
}
