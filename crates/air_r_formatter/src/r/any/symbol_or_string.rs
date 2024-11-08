//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use air_r_syntax::RSymbolOrString;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRSymbolOrString;
impl FormatRule<RSymbolOrString> for FormatRSymbolOrString {
    type Context = RFormatContext;
    fn fmt(&self, node: &RSymbolOrString, f: &mut RFormatter) -> FormatResult<()> {
        match node {
            RSymbolOrString::RIdentifier(node) => node.format().fmt(f),
            RSymbolOrString::RStringValue(node) => node.format().fmt(f),
        }
    }
}
