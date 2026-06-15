use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, Default, Clone, Copy, Eq, Hash, PartialEq)]
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "kebab-case")
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub enum AssignmentStyle {
    /// # Use `<-`
    #[default]
    Arrow,

    /// # Use `=`
    ///
    /// Note that changing from `<-` to `=` is not always possible. For example, `f(x <-
    /// 5)` can't be rewritten as `f(x = 5)` because that would parse as an argument named
    /// `x`. In these cases, the `<-` is left as is.
    Equal,

    /// # Assignment operators are preserved as is
    Preserve,
}

impl FromStr for AssignmentStyle {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "arrow" => Ok(Self::Arrow),
            "equal" => Ok(Self::Equal),
            "preserve" => Ok(Self::Preserve),
            _ => Err("Unsupported value for this option"),
        }
    }
}

impl Display for AssignmentStyle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AssignmentStyle::Arrow => std::write!(f, "Arrow"),
            AssignmentStyle::Equal => std::write!(f, "Equal"),
            AssignmentStyle::Preserve => std::write!(f, "Preserve"),
        }
    }
}
