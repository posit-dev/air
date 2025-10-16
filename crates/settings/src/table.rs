use crate::SortedNames;
use std::fmt;

/// Function names that are automatically formatted as tables without the need
/// for a `fmt: table` comment.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schemars", schemars(description = ""))]
pub struct Table(SortedNames);

impl Table {
    pub fn new(names: Vec<String>) -> Self {
        Self(SortedNames::new(names))
    }

    pub fn as_slice(&self) -> &[String] {
        self.0.as_slice()
    }

    pub fn contains(&self, name: &str) -> bool {
        self.0.contains(name)
    }
}

impl fmt::Display for Table {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for Table {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: SortedNames = serde::Deserialize::deserialize(deserializer)?;
        Ok(Table(value))
    }
}

#[cfg(test)]
mod tests {
    use crate::Table;
    use anyhow::Context;
    use anyhow::Result;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "kebab-case")]
    struct Options {
        table: Option<Table>,
    }

    #[test]
    fn deserialize() -> Result<()> {
        let options: Options = toml::from_str(
            r#"
table = ["my_function"]
"#,
        )?;

        let skip = vec![String::from("my_function")];
        assert_eq!(options.table, Some(Table::new(skip)));

        Ok(())
    }

    #[test]
    fn deserialize_error() -> Result<()> {
        let result: std::result::Result<Options, toml::de::Error> = toml::from_str(
            r"
table = 400
",
        );
        let error = result.err().context("Expected `Table` error")?;
        insta::assert_snapshot!(error);
        Ok(())
    }

    #[test]
    fn display() -> Result<()> {
        let options: Options = toml::from_str(
            r#"
table = ["my_function", "my_other_function"]
"#,
        )?;
        insta::assert_snapshot!(options.table.unwrap());

        let options: Options = toml::from_str(
            r#"
table = []
"#,
        )?;
        insta::assert_snapshot!(options.table.unwrap());

        Ok(())
    }
}
