use std::fmt;
use std::num::NonZeroU16;

/// Validated value for the `line-length` formatter options
///
/// The allowed range of values is 1..=320
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct LineLength(NonZeroU16);

impl LineLength {
    /// Maximum allowed value for a valid [LineLength]
    const MAX: u16 = 320;

    /// Return the numeric value for this [LineLength]
    pub fn value(&self) -> u16 {
        self.0.get()
    }
}

impl Default for LineLength {
    fn default() -> Self {
        Self(NonZeroU16::new(80).unwrap())
    }
}

impl std::fmt::Debug for LineLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        std::fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for LineLength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl<'de> serde::Deserialize<'de> for LineLength {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value: u16 = serde::Deserialize::deserialize(deserializer)?;
        let line_length = LineLength::try_from(value).map_err(serde::de::Error::custom)?;
        Ok(line_length)
    }
}

/// Error type returned when converting a u16 or NonZeroU16 to a [`LineLength`] fails
#[derive(Clone, Copy, Debug)]
pub struct LineLengthFromIntError(u16);

impl std::error::Error for LineLengthFromIntError {}

impl TryFrom<u16> for LineLength {
    type Error = LineLengthFromIntError;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match NonZeroU16::try_from(value) {
            Ok(value) => LineLength::try_from(value),
            Err(_) => Err(LineLengthFromIntError(value)),
        }
    }
}

impl TryFrom<NonZeroU16> for LineLength {
    type Error = LineLengthFromIntError;

    fn try_from(value: NonZeroU16) -> Result<Self, Self::Error> {
        if value.get() <= Self::MAX {
            Ok(LineLength(value))
        } else {
            Err(LineLengthFromIntError(value.get()))
        }
    }
}

impl std::fmt::Display for LineLengthFromIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "The line length must be a value between 1 and {max}, not {value}.",
            max = LineLength::MAX,
            value = self.0
        )
    }
}

impl From<LineLength> for u16 {
    fn from(value: LineLength) -> Self {
        value.0.get()
    }
}

impl From<LineLength> for NonZeroU16 {
    fn from(value: LineLength) -> Self {
        value.0
    }
}

impl From<LineLength> for biome_formatter::LineWidth {
    fn from(value: LineLength) -> Self {
        // Unwrap: We assert that we match biome's `LineWidth` perfectly
        biome_formatter::LineWidth::try_from(value.value()).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Context;
    use anyhow::Result;

    use crate::settings::LineLength;

    #[derive(serde::Deserialize)]
    #[serde(deny_unknown_fields, rename_all = "kebab-case")]
    struct Options {
        line_length: Option<LineLength>,
    }

    #[test]
    fn deserialize_line_length() -> Result<()> {
        let options: Options = toml::from_str(
            r"
line-length = 50
",
        )?;

        assert_eq!(options.line_length, Some(LineLength::try_from(50).unwrap()));

        Ok(())
    }

    #[test]
    fn deserialize_oob_line_length() -> Result<()> {
        let result: std::result::Result<Options, toml::de::Error> = toml::from_str(
            r"
line-length = 400
",
        );
        let error = result.err().context("Expected OOB `LineLength` error")?;
        insta::assert_snapshot!(error);
        Ok(())
    }
}
