//
// line_width.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use std::fmt;
use std::num::NonZeroU16;

/// Validated value for the `line-width` formatter options
///
/// The allowed range of values is 1..=320
#[derive(Clone, Copy, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct LineWidth(
    #[cfg_attr(feature = "schemars", schemars(range(min = 1, max = 320)))] NonZeroU16,
);

impl LineWidth {
    /// Default value for [LineWidth]
    const DEFAULT: u16 = 80;

    /// Maximum allowed value for a valid [LineWidth]
    const MAX: u16 = 320;

    /// Return the numeric value for this [LineWidth]
    pub fn value(&self) -> u16 {
        self.0.get()
    }
}

impl Default for LineWidth {
    fn default() -> Self {
        Self(NonZeroU16::new(Self::DEFAULT).unwrap())
    }
}

impl std::fmt::Debug for LineWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for LineWidth {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[cfg(feature = "serde")]
impl<'de> serde::Deserialize<'de> for LineWidth {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: u16 = serde::Deserialize::deserialize(deserializer)?;
        let line_width = LineWidth::try_from(value).map_err(serde::de::Error::custom)?;
        Ok(line_width)
    }
}

/// Error type returned when converting a u16 or NonZeroU16 to a [`LineWidth`] fails
#[derive(Clone, Copy, Debug)]
pub struct LineWidthFromIntError(u16);

impl std::error::Error for LineWidthFromIntError {}

impl TryFrom<u16> for LineWidth {
    type Error = LineWidthFromIntError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match NonZeroU16::try_from(value) {
            Ok(value) => LineWidth::try_from(value),
            Err(_) => Err(LineWidthFromIntError(value)),
        }
    }
}

impl TryFrom<NonZeroU16> for LineWidth {
    type Error = LineWidthFromIntError;

    fn try_from(value: NonZeroU16) -> Result<Self, Self::Error> {
        if value.get() <= Self::MAX {
            Ok(LineWidth(value))
        } else {
            Err(LineWidthFromIntError(value.get()))
        }
    }
}

impl std::fmt::Display for LineWidthFromIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "The line width must be a value between 1 and {max}, not {value}.",
            max = LineWidth::MAX,
            value = self.0
        )
    }
}

impl From<LineWidth> for u16 {
    fn from(value: LineWidth) -> Self {
        value.0.get()
    }
}

impl From<LineWidth> for NonZeroU16 {
    fn from(value: LineWidth) -> Self {
        value.0
    }
}

#[cfg(feature = "biome")]
impl From<LineWidth> for biome_formatter::LineWidth {
    fn from(value: LineWidth) -> Self {
        // Unwrap: We assert that we match biome's `LineWidth` perfectly
        biome_formatter::LineWidth::try_from(value.value()).unwrap()
    }
}

#[cfg(feature = "biome")]
impl From<biome_formatter::LineWidth> for LineWidth {
    fn from(value: biome_formatter::LineWidth) -> Self {
        // Unwrap: We assert that we match biome's `LineWidth` perfectly
        LineWidth::try_from(value.value()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use anyhow::Result;

    use crate::LineWidth;

    #[test]
    fn to_biome_conversion() {
        assert_eq!(
            biome_formatter::LineWidth::from(LineWidth::try_from(LineWidth::MAX).unwrap()),
            biome_formatter::LineWidth::try_from(LineWidth::MAX).unwrap()
        );
    }

    #[test]
    fn from_biome_conversion() {
        assert_eq!(
            LineWidth::from(biome_formatter::LineWidth::try_from(LineWidth::MAX).unwrap()),
            LineWidth::try_from(LineWidth::MAX).unwrap()
        )
    }

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "kebab-case")]
    struct Options {
        line_width: Option<LineWidth>,
    }

    #[test]
    fn deserialize_line_width() -> Result<()> {
        let options: Options = toml::from_str(
            r"
line-width = 50
",
        )?;

        assert_eq!(options.line_width, Some(LineWidth::try_from(50).unwrap()));

        Ok(())
    }

    #[test]
    fn deserialize_oob_line_width() -> Result<()> {
        let result: std::result::Result<Options, toml::de::Error> = toml::from_str(
            r"
line-width = 400
",
        );
        let error = result.err().context("Expected OOB `LineWidth` error")?;
        insta::assert_snapshot!(error);
        Ok(())
    }
}
