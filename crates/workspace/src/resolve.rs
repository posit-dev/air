use std::collections::btree_map::Keys;
use std::collections::btree_map::Range;
use std::collections::btree_map::RangeMut;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use ignore::DirEntry;
use rustc_hash::FxHashSet;
use thiserror::Error;

use crate::settings::Settings;
use crate::toml::find_air_toml_in_directory;
use crate::toml::parse_air_toml;
use crate::toml::ParseTomlError;

/// Resolves a [`Path`] to its associated `T`
///
/// To use a [`PathResolver`]:
/// - Load directories into it using [`PathResolver::add()`]
/// - Resolve a [`Path`] to its associated `T` with [`PathResolver::resolve()`]
///
/// See [`PathResolver::resolve()`] for more details on the implementation.
#[derive(Debug, Default)]
pub struct PathResolver<T> {
    /// Fallback value to be used when a `path` isn't associated with anything in the `map`
    fallback: T,

    /// An ordered `BTreeMap` from a `path` (normally, a directory) to a `T`
    map: BTreeMap<PathBuf, T>,
}

impl<T> PathResolver<T> {
    /// Create a new empty [`PathResolver`]
    pub fn new(fallback: T) -> Self {
        Self {
            fallback,
            map: BTreeMap::new(),
        }
    }

    pub fn fallback(&self) -> &T {
        &self.fallback
    }

    pub fn add(&mut self, path: &Path, value: T) -> Option<T> {
        self.map.insert(path.to_path_buf(), value)
    }

    pub fn remove(&mut self, path: &Path) -> Option<T> {
        self.map.remove(path)
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    pub fn keys(&self) -> Keys<'_, PathBuf, T> {
        self.map.keys()
    }

    pub fn clear(&mut self) {
        self.map.clear();
    }

    /// Resolve a [`Path`] to its associated `T`
    ///
    /// This resolver works by finding the closest directory to the `path` to search for.
    ///
    /// The [`BTreeMap`] is an ordered map, so if you do:
    ///
    /// ```text
    /// resolver.add("a/b", value1)
    /// resolver.add("a/b/c", value2)
    /// resolver.add("a/b/d", value3)
    /// resolver.resolve("a/b/c/test.R")
    /// ```
    ///
    /// Then it detects both `"a/b"` and `"a/b/c"` as being "less than" the path of
    /// `"a/b/c/test.R"`, and then chooses `"a/b/c"` because it is at the back of
    /// that returned sorted list (i.e. the "closest" match).
    pub fn resolve(&self, path: &Path) -> Option<&T> {
        self.resolve_entry(path).map(|(_, value)| value)
    }

    /// Same as `resolve()`, but returns the internal `fallback` if no associated value
    /// is found.
    pub fn resolve_or_fallback(&self, path: &Path) -> &T {
        self.resolve(path).unwrap_or(self.fallback())
    }

    /// Same as `resolve()`, but returns the `(key, value)` pair.
    ///
    /// Useful when you need the matched workspace path
    pub fn resolve_entry(&self, path: &Path) -> Option<(&PathBuf, &T)> {
        self.matches(path).next_back()
    }

    /// Returns all matches matched by the `path` rather than just the closest one
    pub fn matches(&self, path: &Path) -> Range<'_, PathBuf, T> {
        self.map.range(..path.to_path_buf())
    }

    /// Returns all matches matched by the `path` rather than just the closest one
    pub fn matches_mut(&mut self, path: &Path) -> RangeMut<'_, PathBuf, T> {
        self.map.range_mut(..path.to_path_buf())
    }
}

pub type SettingsResolver = PathResolver<Settings>;

#[derive(Debug, Error)]
pub enum SettingsResolverError {
    #[error(transparent)]
    ParseToml(#[from] ParseTomlError),
}

impl SettingsResolver {
    /// This is the core function for walking a set of `paths` looking for `air.toml`s
    /// and loading in any directories it finds
    ///
    /// For each `path`, we:
    /// - Walk up its ancestors, looking for an `air.toml`
    /// - TODO(hierarchical): Walk down its children, looking for nested `air.toml`s
    ///
    /// Whenever we find an `air.toml`, we add the directory it was found in and
    /// the parsed [`Settings`] into the resolver.
    pub fn load_from_paths<P: AsRef<Path>>(
        &mut self,
        paths: &[P],
    ) -> Result<(), SettingsResolverError> {
        let paths: Vec<PathBuf> = paths.iter().map(fs::normalize_path).collect();

        let mut seen = FxHashSet::default();

        // Load the `resolver` with `Settings` associated with each `path`
        for path in &paths {
            for ancestor in path.ancestors() {
                if seen.insert(ancestor) {
                    if let Some(toml) = find_air_toml_in_directory(ancestor) {
                        let settings = Self::parse_settings(&toml)?;
                        self.add(ancestor, settings);
                        break;
                    }
                } else {
                    // We already visited this ancestor, we can stop here.
                    break;
                }
            }
        }

        // TODO(hierarchical): Also iterate through the directories and collect `air.toml`
        // found nested withing the directories for hierarchical support

        Ok(())
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
}

/// For each provided `path`, recursively search for any R files within that `path`
/// that match our inclusion criteria
///
/// NOTE: Make sure that the inclusion criteria that guide `path` discovery are also
/// consistently applied to [SettingsResolver::load_from_paths()].
pub fn discover_r_file_paths<P: AsRef<Path>>(paths: &[P]) -> Vec<Result<PathBuf, ignore::Error>> {
    let paths: Vec<PathBuf> = paths.iter().map(fs::normalize_path).collect();

    let Some((first_path, paths)) = paths.split_first() else {
        // No paths provided
        return Vec::new();
    };

    // TODO: Parallel directory visitor
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

    let mut paths = Vec::new();

    // Walk all `paths` recursively, collecting R files that we can format
    for path in builder.build() {
        match path {
            Ok(entry) => {
                if let Some(path) = is_match(entry) {
                    paths.push(Ok(path));
                }
            }
            Err(err) => {
                paths.push(Err(err));
            }
        }
    }

    paths
}

// Decide whether or not to accept an `entry` based on include/exclude rules.
fn is_match(entry: DirEntry) -> Option<PathBuf> {
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
