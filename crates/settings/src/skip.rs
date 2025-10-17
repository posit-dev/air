use crate::SortedStrings;
use std::fmt;

/// Function names that are automatically skipped without the need
/// for a `fmt: skip` comment.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schemars", schemars(description = ""))]
pub struct Skip(SortedStrings);

impl Skip {
    /// Constructs [Skip] from a vector of function names
    ///
    /// Not exposed, as deserialization should be the only way to create this type.
    #[cfg(test)]
    fn new(names: Vec<String>) -> Self {
        Self(SortedStrings::new(names))
    }

    pub fn contains(&self, name: &str) -> bool {
        self.0.contains(name)
    }
}

impl fmt::Display for Skip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Skip {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: SortedStrings = serde::Deserialize::deserialize(deserializer)?;
        Ok(Skip(value))
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
