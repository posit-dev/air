//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use air_r_syntax::AnyRArgumentName;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyRArgumentName;
impl FormatRule<AnyRArgumentName> for FormatAnyRArgumentName {
    type Context = RFormatContext;
    fn fmt(&self, node: &AnyRArgumentName, f: &mut RFormatter) -> FormatResult<()> {
        match node {
            AnyRArgumentName::RDots(node) => node.format().fmt(f),
            AnyRArgumentName::RIdentifier(node) => node.format().fmt(f),
            AnyRArgumentName::RStringValue(node) => node.format().fmt(f),
        }
    }
}
