//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use air_r_syntax::AnyRArgumentListElement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyRArgumentListElement;
impl FormatRule<AnyRArgumentListElement> for FormatAnyRArgumentListElement {
    type Context = RFormatContext;
    fn fmt(&self, node: &AnyRArgumentListElement, f: &mut RFormatter) -> FormatResult<()> {
        match node {
            AnyRArgumentListElement::AnyRArgument(node) => node.format().fmt(f),
            AnyRArgumentListElement::RComma(node) => node.format().fmt(f),
        }
    }
}
