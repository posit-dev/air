use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Default, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LineEnding {
    /// The newline style is detected automatically on a file per file basis.
    /// Files with mixed line endings will be converted to the first detected line ending.
    /// Defaults to [`LineEnding::Lf`] for a files that contain no line endings.
    #[default]
    Auto,

    ///  Line endings will be converted to `\n` as is common on Unix.
    Lf,

    /// Line endings will be converted to `\r\n` as is common on Windows.
    Crlf,

    /// Line endings will be converted to `\n` on Unix and `\r\n` on Windows.
    Native,
}

impl fmt::Display for LineEnding {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Auto => write!(f, "auto"),
            Self::Lf => write!(f, "lf"),
            Self::Crlf => write!(f, "crlf"),
            Self::Native => write!(f, "native"),
        }
    }
}
