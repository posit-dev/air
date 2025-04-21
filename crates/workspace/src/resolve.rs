//
// resolve.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

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
    /// Fallback value to be used when a `path` isn't associated with any `items`
    fallback: T,

    /// A sorted vector of `PathItem`. Sorted on the `PathBuf` in increasing order.
    items: Vec<PathItem<T>>,
}

#[derive(Debug, Default)]
pub struct PathItem<T> {
    /// The `path` that is used for "starts with" matching
    path: PathBuf,

    /// The `value` associated with this `path`
    value: T,
}

impl<T> PathItem<T> {
    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn value(&self) -> &T {
        &self.value
    }

    pub fn value_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T> PathResolver<T> {
    /// Create a new empty [`PathResolver`]
    pub fn new(fallback: T) -> Self {
        Self {
            fallback,
            items: Vec::new(),
        }
    }

    pub fn fallback(&self) -> &T {
        &self.fallback
    }

    /// Add a `path` and its `value` into the resolver
    ///
    /// The new item is inserted in such a way that the overall vector remains sorted.
    /// Note that this can be `O(n)` due to the insertion, but we don't anticipate this
    /// being called very often (in practice, there are very few workspaces open, and very
    /// few `air.toml` files to look at).
    ///
    /// If the `path` already exists, the new `value` is inserted and the old one
    /// is returned, otherwise `None` is returned.
    pub fn add<P: Into<PathBuf>>(&mut self, path: P, value: T) -> Option<T> {
        let path = path.into();

        match self.items.binary_search_by(|item| item.path.cmp(&path)) {
            Ok(index) => {
                // `path` already exists! Swap underlying `value` for the new one, return the old one.
                let item = &mut self.items[index];
                let value = std::mem::replace(&mut item.value, value);
                Some(value)
            }
            Err(index) => {
                // `path` is new! Insert new `PathItem` at sorted index to retain overall
                // ordering.
                let item = PathItem { path, value };
                self.items.insert(index, item);
                None
            }
        }
    }

    /// Remove a `path` and its `value` from the resolver
    ///
    /// Returns `Some(value)` if the `path` existed, otherwise returns `None`.
    pub fn remove<P: AsRef<Path>>(&mut self, path: P) -> Option<T> {
        let path = path.as_ref();

        match self
            .items
            .binary_search_by(|item| item.path.as_path().cmp(path))
        {
            Ok(index) => {
                // `path` exists! Remove it and return value.
                let item = self.items.remove(index);
                Some(item.value)
            }
            Err(_) => {
                // `path` does not exist!
                None
            }
        }
    }

    pub fn items(&self) -> &[PathItem<T>] {
        &self.items
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn clear(&mut self) {
        self.items.clear();
    }

    /// Resolve a [`Path`] to its associated `T`
    ///
    /// This resolver works by finding the closest directory to the `path` to search for.
    ///
    /// The internal vector of items is ordered, so if you do:
    ///
    /// ```text
    /// resolver.add("a/b", value1)
    /// resolver.add("a/b/c", value2)
    /// resolver.add("a/b/d", value3)
    /// resolver.resolve("a/b/c/test.R")
    /// ```
    ///
    /// Then we iterate from the end and find the first [PathItem] that forms a prefix for
    /// the `path` of interest. Because the vector is sorted and we start from the back,
    /// this will give us the longest path, i.e. the "closest" one to `path`.
    ///
    /// The [std::path::Path::starts_with()] method knows that `a/b` is NOT a prefix for
    /// `a/b.R`, which aligns with our needs.
    pub fn resolve<P: AsRef<Path>>(&self, path: P) -> Option<&PathItem<T>> {
        let path = path.as_ref();

        self.items
            .iter()
            .rev()
            .find(|item| path.starts_with(item.path()))
    }

    /// Convenience method when you don't care about manually handling the fallback
    /// case and don't need the matched path in the tree
    pub fn resolve_or_fallback<P: AsRef<Path>>(&self, path: P) -> &T {
        self.resolve(path)
            .map_or_else(|| self.fallback(), |item| item.value())
    }

    /// Returns all matches matched by the `path` rather than just the closest one
    ///
    /// See [PathResolver::resolve()] for implementation details.
    ///
    /// Requires an owned [PathBuf] on input so that we can return a lazy iterator
    /// that uses it.
    pub fn matches<P: Into<PathBuf>>(&self, path: P) -> impl Iterator<Item = &PathItem<T>> {
        let path: PathBuf = path.into();

        self.items
            .iter()
            .filter(move |item| path.starts_with(item.path()))
    }

    /// Returns all matches matched by the `path` rather than just the closest one
    ///
    /// See [PathResolver::resolve()] for implementation details.
    ///
    /// Requires an owned [PathBuf] on input so that we can return a lazy iterator
    /// that uses it.
    pub fn matches_mut<P: Into<PathBuf>>(
        &mut self,
        path: P,
    ) -> impl Iterator<Item = &mut PathItem<T>> {
        let path: PathBuf = path.into();

        self.items
            .iter_mut()
            .filter(move |item| path.starts_with(item.path()))
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::resolve::PathResolver;

    #[test]
    fn test_items_are_added_and_removed_in_sorted_order() {
        let mut resolver = PathResolver::new(String::from("fallback"));

        resolver.add("/user/c", String::from("c"));
        resolver.add("/user/a", String::from("a"));
        resolver.add("/user/b", String::from("b"));

        let items = resolver.items();
        assert_eq!(items[0].value(), &String::from("a"));
        assert_eq!(items[1].value(), &String::from("b"));
        assert_eq!(items[2].value(), &String::from("c"));

        resolver.remove("/user/b");

        let items = resolver.items();
        assert_eq!(items[0].value(), &String::from("a"));
        assert_eq!(items[1].value(), &String::from("c"));
    }

    #[test]
    fn test_matches_must_be_strict_prefixes() {
        let mut resolver = PathResolver::new(String::from("fallback"));
        resolver.add("/user/a", String::from("a"));
        resolver.add("/user/b", String::from("b"));
        resolver.add("/user/b/c", String::from("c"));

        // Even though `"/user/a" < "/user/b/c/foo.R"` lexicographically, we should not
        // return it as a match
        let mut matches = resolver.matches("/user/b/c/foo.R");

        let item = matches.next().unwrap();
        assert_eq!(item.value, String::from("b"));

        let item = matches.next().unwrap();
        assert_eq!(item.value, String::from("c"));

        assert!(matches.next().is_none());
    }

    #[test]
    fn test_starts_with_strategy_only_matches_full_components() {
        let mut resolver = PathResolver::new(0);
        resolver.add("/user/a", 1);

        // Technically `/user/a.R` "starts with" `/user/a`, but `path.starts_with()`
        // is smart enough to only look at full components
        assert!(resolver.resolve("/user/a.R").is_none());
    }

    #[test]
    fn test_can_resolve_in_simple_cases() {
        let mut resolver = PathResolver::new(0);
        resolver.add("/user/a", 1);
        resolver.add("/user/b", 2);

        let item = resolver.resolve("/user/a/foo.R").unwrap();
        assert_eq!(item.path(), PathBuf::from("/user/a").as_path());
        assert_eq!(item.value(), &1);

        let item = resolver.resolve("/user/b/foo.R").unwrap();
        assert_eq!(item.path(), PathBuf::from("/user/b").as_path());
        assert_eq!(item.value(), &2);
    }

    #[test]
    fn test_resolves_to_closest_path() {
        let mut resolver = PathResolver::new(0);
        resolver.add("/user/b", 1);
        resolver.add("/user/b/a", 2);

        let item = resolver.resolve("/user/b/foo.R").unwrap();
        assert_eq!(item.path(), PathBuf::from("/user/b").as_path());

        let item = resolver.resolve("/user/b/b/foo.R").unwrap();
        assert_eq!(item.path(), PathBuf::from("/user/b").as_path());

        let item = resolver.resolve("/user/b/a/foo.R").unwrap();
        assert_eq!(item.path(), PathBuf::from("/user/b/a").as_path());

        assert!(resolver.resolve("/user/a/a/foo.R").is_none());
    }
}
