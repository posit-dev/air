use std::fmt;
use std::num::NonZeroU8;

/// Validated value for the `indent-width` formatter options
///
/// The allowed range of values is 1..=24
#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct IndentWidth(NonZeroU8);

impl IndentWidth {
    /// Maximum allowed value for a valid [IndentWidth]
    const MAX: u8 = 24;

    /// Return the numeric value for this [IndentWidth]
    pub fn value(&self) -> u8 {
        self.0.get()
    }
}

impl Default for IndentWidth {
    fn default() -> Self {
        Self(NonZeroU8::new(4).unwrap())
    }
}

impl std::fmt::Debug for IndentWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl std::fmt::Display for IndentWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl<'de> serde::Deserialize<'de> for IndentWidth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: u8 = serde::Deserialize::deserialize(deserializer)?;
        let indent_width = IndentWidth::try_from(value).map_err(serde::de::Error::custom)?;
        Ok(indent_width)
    }
}

/// Error type returned when converting a u8 or NonZeroU8 to a [`IndentWidth`] fails
#[derive(Clone, Copy, Debug)]
pub struct IndentWidthFromIntError(u8);

impl std::error::Error for IndentWidthFromIntError {}

impl TryFrom<u8> for IndentWidth {
    type Error = IndentWidthFromIntError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match NonZeroU8::try_from(value) {
            Ok(value) => IndentWidth::try_from(value),
            Err(_) => Err(IndentWidthFromIntError(value)),
        }
    }
}

impl TryFrom<NonZeroU8> for IndentWidth {
    type Error = IndentWidthFromIntError;

    fn try_from(value: NonZeroU8) -> Result<Self, Self::Error> {
        if value.get() <= Self::MAX {
            Ok(IndentWidth(value))
        } else {
            Err(IndentWidthFromIntError(value.get()))
        }
    }
}

impl std::fmt::Display for IndentWidthFromIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "The indent width must be a value between 1 and {max}, not {value}.",
            max = IndentWidth::MAX,
            value = self.0
        )
    }
}

impl From<IndentWidth> for u8 {
    fn from(value: IndentWidth) -> Self {
        value.0.get()
    }
}

impl From<IndentWidth> for NonZeroU8 {
    fn from(value: IndentWidth) -> Self {
        value.0
    }
}

impl From<IndentWidth> for biome_formatter::IndentWidth {
    fn from(value: IndentWidth) -> Self {
        // Unwrap: We assert that we match biome's `IndentWidth` perfectly
        biome_formatter::IndentWidth::try_from(value.value()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use anyhow::Result;

    use crate::settings::IndentWidth;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "kebab-case")]
    struct Options {
        indent_width: Option<IndentWidth>,
    }

    #[test]
    fn deserialize_indent_width() -> Result<()> {
        let options: Options = toml::from_str(
            r"
indent-width = 2
",
        )?;

        assert_eq!(
            options.indent_width,
            Some(IndentWidth::try_from(2).unwrap())
        );

        Ok(())
    }

    #[test]
    fn deserialize_oob_indent_width() -> Result<()> {
        let result: std::result::Result<Options, toml::de::Error> = toml::from_str(
            r"
indent-width = 25
",
        );
        let error = result.err().context("Expected OOB `IndentWidth` error")?;
        insta::assert_snapshot!(error);
        Ok(())
    }
}
