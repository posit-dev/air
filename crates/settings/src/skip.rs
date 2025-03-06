//
// skip.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use std::fmt::Display;
use std::sync::Arc;

/// Function names that are automatically skipped without the need
/// for a `fmt: skip` comment.
///
/// # Notes
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
pub struct Skip(Arc<[String]>);

impl Skip {
    /// Constructs [Skip] from a vector of function names
    ///
    /// Not exposed, as deserialization should be the only way to create this type.
    fn new(mut names: Vec<String>) -> Self {
        names.sort_unstable();
        Self(names.into())
    }

    /// Checks if `name` is contained in the list of function names to skip
    pub fn contains(&self, name: &str) -> bool {
        self.0
            .binary_search_by(|probe| probe.as_str().cmp(name))
            .is_ok()
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Skip {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: Vec<String> = serde::Deserialize::deserialize(deserializer)?;
        let value = Skip::new(value);
        Ok(value)
    }
}

impl Display for Skip {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use anyhow::Result;

    use crate::Skip;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "kebab-case")]
    struct Options {
        skip: Option<Skip>,
    }

    #[test]
    fn deserialize() -> Result<()> {
        let options: Options = toml::from_str(
            r#"
skip = ["my_function"]
"#,
        )?;

        let skip = vec![String::from("my_function")];
        assert_eq!(options.skip, Some(Skip::new(skip)));

        Ok(())
    }

    #[test]
    fn deserialize_error() -> Result<()> {
        let result: std::result::Result<Options, toml::de::Error> = toml::from_str(
            r"
skip = 400
",
        );
        let error = result.err().context("Expected `Skip` error")?;
        insta::assert_snapshot!(error);
        Ok(())
    }

    #[test]
    fn display() -> Result<()> {
        let options: Options = toml::from_str(
            r#"
skip = ["my_function", "my_other_function"]
"#,
        )?;
        insta::assert_snapshot!(options.skip.unwrap());

        let options: Options = toml::from_str(
            r#"
skip = []
"#,
        )?;
        insta::assert_snapshot!(options.skip.unwrap());

        Ok(())
    }
}
