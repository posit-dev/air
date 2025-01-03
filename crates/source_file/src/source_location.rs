use std::fmt::{Debug, Formatter};

use crate::OneIndexed;

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct SourceLocation {
    pub row: OneIndexed,
    pub column: OneIndexed,
}

impl Default for SourceLocation {
    fn default() -> Self {
        Self {
            row: OneIndexed::MIN,
            column: OneIndexed::MIN,
        }
    }
}

impl Debug for SourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceLocation")
            .field("row", &self.row.get())
            .field("column", &self.column.get())
            .finish()
    }
}

impl std::fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{row}:{column}", row = self.row, column = self.column)
    }
}
