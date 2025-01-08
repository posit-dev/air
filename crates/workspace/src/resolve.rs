//
// resolve.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use std::collections::btree_map::Keys;
use std::collections::btree_map::Range;
use std::collections::btree_map::RangeMut;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

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

pub struct PathResolution<'resolver, T> {
    /// The `path` in the tree that was closest to the path provided in [`PathResolver::resolve`]
    path: &'resolver PathBuf,

    /// The `value` in the tree that matches the path provided in [`PathResolver::resolve`]
    value: &'resolver T,
}

impl<'resolver, T> PathResolution<'resolver, T> {
    pub fn path(&self) -> &'resolver PathBuf {
        self.path
    }

    pub fn value(&self) -> &'resolver T {
        self.value
    }
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
    pub fn resolve(&self, path: &Path) -> Option<PathResolution<T>> {
        self.matches(path)
            .next_back()
            .map(|(path, value)| PathResolution { path, value })
    }

    /// Convenience method when you don't care about manually handling the fallback
    /// case and don't need the matched path in the tree
    pub fn resolve_or_fallback(&self, path: &Path) -> &T {
        self.resolve(path)
            .map_or_else(|| self.fallback(), |resolution| resolution.value())
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
