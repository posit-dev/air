use crate::prelude::*;
use crate::{AsFormat, IntoFormat, RFormatContext};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult};
use biome_r_syntax::{map_syntax_node, RSyntaxNode};

#[derive(Debug, Copy, Clone, Default)]
pub struct FormatRSyntaxNode;

impl FormatRule<RSyntaxNode> for FormatRSyntaxNode {
    type Context = RFormatContext;

    fn fmt(&self, node: &RSyntaxNode, f: &mut RFormatter) -> FormatResult<()> {
        map_syntax_node!(node.clone(), node => node.format().fmt(f))
    }
}

impl AsFormat<RFormatContext> for RSyntaxNode {
    type Format<'a> = FormatRefWithRule<'a, RSyntaxNode, FormatRSyntaxNode>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatRSyntaxNode)
    }
}

impl IntoFormat<RFormatContext> for RSyntaxNode {
    type Format = FormatOwnedWithRule<RSyntaxNode, FormatRSyntaxNode>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatRSyntaxNode)
    }
}
