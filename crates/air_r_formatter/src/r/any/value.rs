//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use air_r_syntax::AnyRValue;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyRValue;
impl FormatRule<AnyRValue> for FormatAnyRValue {
    type Context = RFormatContext;
    fn fmt(&self, node: &AnyRValue, f: &mut RFormatter) -> FormatResult<()> {
        match node {
            AnyRValue::RBogusValue(node) => node.format().fmt(f),
            AnyRValue::RComplexValue(node) => node.format().fmt(f),
            AnyRValue::RDoubleValue(node) => node.format().fmt(f),
            AnyRValue::RIntegerValue(node) => node.format().fmt(f),
            AnyRValue::RStringValue(node) => node.format().fmt(f),
        }
    }
}
