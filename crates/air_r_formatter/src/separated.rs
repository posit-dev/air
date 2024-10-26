use crate::prelude::*;
use crate::{AsFormat, FormatRSyntaxToken};
use biome_formatter::separated::{FormatSeparatedElementRule, FormatSeparatedIter};
use biome_formatter::FormatRefWithRule;
use air_r_syntax::{RLanguage, RSyntaxToken};
use biome_rowan::{AstNode, AstSeparatedList, AstSeparatedListElementsIterator};
use std::marker::PhantomData;

#[derive(Clone)]
pub(crate) struct RFormatSeparatedElementRule<N>
where
    N: AstNode<Language = RLanguage>,
{
    node: PhantomData<N>,
}

impl<N> FormatSeparatedElementRule<N> for RFormatSeparatedElementRule<N>
where
    N: AstNode<Language = RLanguage> + AsFormat<RFormatContext> + 'static,
{
    type Context = RFormatContext;
    type FormatNode<'a> = N::Format<'a>;
    type FormatSeparator<'a> = FormatRefWithRule<'a, RSyntaxToken, FormatRSyntaxToken>;

    fn format_node<'a>(&self, node: &'a N) -> Self::FormatNode<'a> {
        node.format()
    }

    fn format_separator<'a>(&self, separator: &'a RSyntaxToken) -> Self::FormatSeparator<'a> {
        separator.format()
    }
}

type RFormatSeparatedIter<Node> = FormatSeparatedIter<
    AstSeparatedListElementsIterator<RLanguage, Node>,
    Node,
    RFormatSeparatedElementRule<Node>,
>;

/// AST Separated list formatting extension methods
pub(crate) trait FormatAstSeparatedListExtension:
    AstSeparatedList<Language = RLanguage>
{
    /// Prints a separated list of nodes
    ///
    /// Trailing separators will be reused from the original list or
    /// created by calling the `separator_factory` function.
    /// The last trailing separator in the list will only be printed
    /// if the outer group breaks.
    fn format_separated(&self, separator: &'static str) -> RFormatSeparatedIter<Self::Node> {
        RFormatSeparatedIter::new(
            self.elements(),
            separator,
            RFormatSeparatedElementRule { node: PhantomData },
        )
    }
}

impl<T> FormatAstSeparatedListExtension for T where T: AstSeparatedList<Language = RLanguage> {}
