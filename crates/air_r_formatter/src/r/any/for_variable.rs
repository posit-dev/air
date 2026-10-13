//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use air_r_syntax::AnyRForVariable;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyRForVariable;
impl FormatRule<AnyRForVariable> for FormatAnyRForVariable {
    type Context = RFormatContext;
    fn fmt(&self, node: &AnyRForVariable, f: &mut RFormatter) -> FormatResult<()> {
        match node {
            AnyRForVariable::RBogusForVariable(node) => node.format().fmt(f),
            AnyRForVariable::RDotDotI(node) => node.format().fmt(f),
            AnyRForVariable::RDots(node) => node.format().fmt(f),
            AnyRForVariable::RIdentifier(node) => node.format().fmt(f),
        }
    }
}
