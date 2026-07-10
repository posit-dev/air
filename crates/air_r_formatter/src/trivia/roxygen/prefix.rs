use biome_formatter::Format;
use biome_formatter::FormatResult;
use biome_formatter::prelude::text;

use crate::RFormatter;
use crate::context::RFormatContext;

#[derive(Debug, Clone, Copy)]
pub(crate) enum RoxygenPrefix {
    /// `#'`
    Single,

    /// `##'`
    Double,
}

impl RoxygenPrefix {
    /// Length of the prefix, i.e. of `#'` or `##'`
    pub(crate) fn len(&self) -> u8 {
        match self {
            RoxygenPrefix::Single => 2,
            RoxygenPrefix::Double => 3,
        }
    }
}

impl Format<RFormatContext> for RoxygenPrefix {
    fn fmt(&self, f: &mut RFormatter) -> FormatResult<()> {
        // `text()` takes a `&' static str` that is never copied, so is very efficient for
        // repeated usage
        match self {
            RoxygenPrefix::Single => text("#'").fmt(f),
            RoxygenPrefix::Double => text("##'").fmt(f),
        }
    }
}
