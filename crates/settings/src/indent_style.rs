//
// indent_style.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, Eq, Hash, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "kebab-case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum IndentStyle {
    /// Tab
    Tab,
    /// Space
    #[default]
    Space,
}

impl IndentStyle {
    /// Returns `true` if this is an [IndentStyle::Tab].
    pub const fn is_tab(&self) -> bool {
        matches!(self, IndentStyle::Tab)
    }

    /// Returns `true` if this is an [IndentStyle::Space].
    pub const fn is_space(&self) -> bool {
        matches!(self, IndentStyle::Space)
    }
}

impl FromStr for IndentStyle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "tab" => Ok(Self::Tab),
            "space" => Ok(Self::Space),
            _ => Err("Unsupported value for this option"),
        }
    }
}

impl Display for IndentStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndentStyle::Tab => std::write!(f, "Tab"),
            IndentStyle::Space => std::write!(f, "Space"),
        }
    }
}

#[cfg(feature = "biome")]
impl From<IndentStyle> for biome_formatter::IndentStyle {
    fn from(value: IndentStyle) -> Self {
        match value {
            IndentStyle::Tab => biome_formatter::IndentStyle::Tab,
            IndentStyle::Space => biome_formatter::IndentStyle::Space,
        }
    }
}

#[cfg(feature = "biome")]
impl From<biome_formatter::IndentStyle> for IndentStyle {
    fn from(value: biome_formatter::IndentStyle) -> Self {
        match value {
            biome_formatter::IndentStyle::Tab => IndentStyle::Tab,
            biome_formatter::IndentStyle::Space => IndentStyle::Space,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::IndentStyle;

    #[test]
    fn to_biome_conversion() {
        assert_eq!(
            biome_formatter::IndentStyle::from(IndentStyle::Tab),
            biome_formatter::IndentStyle::Tab
        );
        assert_eq!(
            biome_formatter::IndentStyle::from(IndentStyle::Space),
            biome_formatter::IndentStyle::Space
        );
    }

    #[test]
    fn from_biome_conversion() {
        assert_eq!(
            IndentStyle::from(biome_formatter::IndentStyle::Tab),
            IndentStyle::Tab
        );
        assert_eq!(
            IndentStyle::from(biome_formatter::IndentStyle::Space),
            IndentStyle::Space
        );
    }
}
