//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use air_r_syntax::AnyRStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyRStatement;
impl FormatRule<AnyRStatement> for FormatAnyRStatement {
    type Context = RFormatContext;
    fn fmt(&self, node: &AnyRStatement, f: &mut RFormatter) -> FormatResult<()> {
        match node {
            AnyRStatement::REmptyStatement(node) => node.format().fmt(f),
            AnyRStatement::RExpressionStatement(node) => node.format().fmt(f),
        }
    }
}
