//
// line_ending.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum LineEnding {
    /// The newline style is detected automatically on a file per file basis. Files with
    /// mixed line endings will be converted to the first detected line ending. Defaults
    /// to `\n` for a files that contain no line endings.
    #[default]
    Auto,

    /// Line endings will be converted to `\n` as is common on Unix.
    Lf,

    /// Line endings will be converted to `\r\n` as is common on Windows.
    Crlf,

    /// Line endings will be converted to `\n` on Unix and `\r\n` on Windows.
    Native,
}

impl fmt::Display for LineEnding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "Auto"),
            Self::Lf => write!(f, "LF"),
            Self::Crlf => write!(f, "CRLF"),
            Self::Native => write!(f, "Native"),
        }
    }
}

impl LineEnding {
    pub(crate) fn finalize(&self, source: &str) -> settings::LineEnding {
        match self {
            LineEnding::Lf => settings::LineEnding::Lf,
            LineEnding::Crlf => settings::LineEnding::Crlf,
            #[cfg(target_os = "windows")]
            LineEnding::Native => settings::LineEnding::Crlf,
            #[cfg(not(target_os = "windows"))]
            LineEnding::Native => settings::LineEnding::Lf,
            LineEnding::Auto => line_ending::infer(source),
        }
    }
}
