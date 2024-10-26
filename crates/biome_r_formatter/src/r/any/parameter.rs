//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use biome_r_syntax::AnyRParameter;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyRParameter;
impl FormatRule<AnyRParameter> for FormatAnyRParameter {
    type Context = RFormatContext;
    fn fmt(&self, node: &AnyRParameter, f: &mut RFormatter) -> FormatResult<()> {
        match node {
            AnyRParameter::RBogusParameter(node) => node.format().fmt(f),
            AnyRParameter::RDefaultParameter(node) => node.format().fmt(f),
            AnyRParameter::RDotsParameter(node) => node.format().fmt(f),
            AnyRParameter::RIdentifierParameter(node) => node.format().fmt(f),
        }
    }
}
