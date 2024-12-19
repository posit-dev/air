use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, Eq, Hash, PartialEq)]
pub enum MagicLineBreak {
    /// Respect
    #[default]
    Respect,
    /// Ignore
    Ignore,
}

impl MagicLineBreak {
    /// Returns `true` if magic line breaks should be respected.
    pub const fn is_respect(&self) -> bool {
        matches!(self, MagicLineBreak::Respect)
    }

    /// Returns `true` if magic line breaks should be ignored.
    pub const fn is_ignore(&self) -> bool {
        matches!(self, MagicLineBreak::Ignore)
    }
}

impl FromStr for MagicLineBreak {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "respect" => Ok(Self::Respect),
            "ignore" => Ok(Self::Ignore),
            _ => Err("Unsupported value for this option"),
        }
    }
}

impl Display for MagicLineBreak {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MagicLineBreak::Respect => std::write!(f, "Respect"),
            MagicLineBreak::Ignore => std::write!(f, "Ignore"),
        }
    }
}
