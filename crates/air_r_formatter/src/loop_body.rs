use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use biome_formatter::format_args;
use biome_formatter::write;

pub(crate) struct FormatLoopBody<'a> {
    node: &'a AnyRExpression,
}

impl<'a> FormatLoopBody<'a> {
    pub fn new(node: &'a AnyRExpression) -> Self {
        Self { node }
    }
}

impl Format<RFormatContext> for FormatLoopBody<'_> {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        if matches!(&self.node, AnyRExpression::RBracedExpressions(_)) {
            // If it's already braced, just write it
            write!(f, [self.node.format()])
        } else {
            // Otherwise, brace it and block indent the body
            write!(
                f,
                [
                    text("{"),
                    block_indent(&format_args![&self.node.format()]),
                    text("}")
                ]
            )
        }
    }
}
