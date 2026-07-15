use std::fmt::Display;

#[derive(Debug, Default, Clone, Copy, Eq, Hash, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RoxygenExamples {
    /// Disabled
    #[default]
    Disabled,

    /// Enabled
    Enabled,
}

impl RoxygenExamples {
    pub const fn is_disabled(&self) -> bool {
        matches!(self, RoxygenExamples::Disabled)
    }

    pub const fn is_enabled(&self) -> bool {
        matches!(self, RoxygenExamples::Enabled)
    }
}

impl Display for RoxygenExamples {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoxygenExamples::Disabled => std::write!(f, "Disabled"),
            RoxygenExamples::Enabled => std::write!(f, "Enabled"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn default_is_disabled() {
        assert!(RoxygenExamples::default().is_disabled());
    }
}
