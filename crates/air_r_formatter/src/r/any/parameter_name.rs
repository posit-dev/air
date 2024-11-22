//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use air_r_syntax::AnyRParameterName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyRParameterName;
impl FormatRule<AnyRParameterName> for FormatAnyRParameterName {
    type Context = RFormatContext;
    fn fmt(&self, node: &AnyRParameterName, f: &mut RFormatter) -> FormatResult<()> {
        match node {
            AnyRParameterName::RDotDotI(node) => node.format().fmt(f),
            AnyRParameterName::RDots(node) => node.format().fmt(f),
            AnyRParameterName::RIdentifier(node) => node.format().fmt(f),
        }
    }
}
