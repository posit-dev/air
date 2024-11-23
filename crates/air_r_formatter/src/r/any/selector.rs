//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use air_r_syntax::AnyRSelector;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyRSelector;
impl FormatRule<AnyRSelector> for FormatAnyRSelector {
    type Context = RFormatContext;
    fn fmt(&self, node: &AnyRSelector, f: &mut RFormatter) -> FormatResult<()> {
        match node {
            AnyRSelector::RDotDotI(node) => node.format().fmt(f),
            AnyRSelector::RDots(node) => node.format().fmt(f),
            AnyRSelector::RIdentifier(node) => node.format().fmt(f),
            AnyRSelector::RStringValue(node) => node.format().fmt(f),
        }
    }
}
