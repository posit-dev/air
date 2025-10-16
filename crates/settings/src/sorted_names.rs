use std::fmt;
use std::sync::Arc;

/// An immutable, sorted list of names (strings), backed by Arc for cheap cloning.
///
/// Internally wrapped in an [Arc] for cheap cloning, since we know the function names
/// are immutable and can be shared. Must be an [Arc] because settings are shared across
/// threads when doing parallel file discovery.
///
/// Clippy recommends [Arc] wrapping the immutable `[String]` over the mutable
/// `Vec<String>` because with [Arc] you are basically promising not to mutate the inner
/// object, and [Arc] provides a special `From<Vec<T>> for Arc<[T]>` for exactly this use
/// case, which we invoke in [Skip::new()].
///
/// # Safety
///
/// This vector is sorted at creation, for use with binary search during lookups.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schemars", schemars(description = ""))]
pub struct SortedNames(Arc<[String]>);

impl SortedNames {
    /// Constructs a new SortedNames, sorting the input for binary search.
    pub fn new(mut names: Vec<String>) -> Self {
        names.sort_unstable();
        Self(names.into())
    }

    /// Returns the sorted slice.
    pub fn as_slice(&self) -> &[String] {
        &self.0
    }

    /// Checks if a name is present (binary search).
    pub fn contains(&self, name: &str) -> bool {
        self.0
            .binary_search_by(|probe| probe.as_str().cmp(name))
            .is_ok()
    }
}

impl fmt::Display for SortedNames {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut names = self.0.iter();
        let last = names.next_back();
        for name in names {
            f.write_str(name)?;
            f.write_str(", ")?;
        }
        if let Some(last) = last {
            f.write_str(last)?;
        }
        Ok(())
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for SortedNames {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Vec<String> = serde::Deserialize::deserialize(deserializer)?;
        Ok(SortedNames::new(value))
    }
}
