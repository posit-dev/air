//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use air_r_syntax::AnyRArgument;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyRArgument;
impl FormatRule<AnyRArgument> for FormatAnyRArgument {
    type Context = RFormatContext;
    fn fmt(&self, node: &AnyRArgument, f: &mut RFormatter) -> FormatResult<()> {
        match node {
            AnyRArgument::RBogusArgument(node) => node.format().fmt(f),
            AnyRArgument::RDotsArgument(node) => node.format().fmt(f),
            AnyRArgument::RNamedArgument(node) => node.format().fmt(f),
            AnyRArgument::RUnnamedArgument(node) => node.format().fmt(f),
        }
    }
}
