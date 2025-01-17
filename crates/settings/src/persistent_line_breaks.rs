//
// persistent_line_breaks.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PersistentLineBreaks {
    /// Respect
    #[default]
    Respect,
    /// Ignore
    Ignore,
}

impl PersistentLineBreaks {
    /// Returns `true` if persistent line breaks should be respected.
    pub const fn is_respect(&self) -> bool {
        matches!(self, PersistentLineBreaks::Respect)
    }

    /// Returns `true` if persistent line breaks should be ignored.
    pub const fn is_ignore(&self) -> bool {
        matches!(self, PersistentLineBreaks::Ignore)
    }
}

impl FromStr for PersistentLineBreaks {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "respect" => Ok(Self::Respect),
            "ignore" => Ok(Self::Ignore),
            _ => Err("Unsupported value for this option"),
        }
    }
}

impl Display for PersistentLineBreaks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PersistentLineBreaks::Respect => std::write!(f, "Respect"),
            PersistentLineBreaks::Ignore => std::write!(f, "Ignore"),
        }
    }
}
