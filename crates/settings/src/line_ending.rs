//
// line_ending.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use std::fmt;

#[derive(Copy, Clone, Default, Debug, Eq, PartialEq)]
pub enum LineEnding {
    ///  Line endings will be converted to `\n` as is common on Unix.
    #[default]
    Lf,

    /// Line endings will be converted to `\r\n` as is common on Windows.
    Crlf,
}

impl fmt::Display for LineEnding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Lf => write!(f, "LF"),
            Self::Crlf => write!(f, "CRLF"),
        }
    }
}

#[cfg(feature = "biome")]
impl From<LineEnding> for biome_formatter::LineEnding {
    fn from(value: LineEnding) -> Self {
        match value {
            LineEnding::Lf => biome_formatter::LineEnding::Lf,
            LineEnding::Crlf => biome_formatter::LineEnding::Crlf,
        }
    }
}

#[cfg(feature = "biome")]
impl From<biome_formatter::LineEnding> for LineEnding {
    fn from(value: biome_formatter::LineEnding) -> Self {
        match value {
            biome_formatter::LineEnding::Lf => LineEnding::Lf,
            biome_formatter::LineEnding::Crlf => LineEnding::Crlf,
            biome_formatter::LineEnding::Cr => panic!("Unsupported `Cr` line endings."),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::LineEnding;

    #[test]
    fn to_biome_conversion() {
        assert_eq!(
            biome_formatter::LineEnding::from(LineEnding::Lf),
            biome_formatter::LineEnding::Lf
        );
        assert_eq!(
            biome_formatter::LineEnding::from(LineEnding::Crlf),
            biome_formatter::LineEnding::Crlf
        );
    }

    #[test]
    fn from_biome_conversion() {
        assert_eq!(
            LineEnding::from(biome_formatter::LineEnding::Lf),
            LineEnding::Lf
        );
        assert_eq!(
            LineEnding::from(biome_formatter::LineEnding::Crlf),
            LineEnding::Crlf
        );
    }

    #[test]
    #[should_panic]
    fn from_biome_conversion_failure() {
        let _ = LineEnding::from(biome_formatter::LineEnding::Cr);
    }
}
