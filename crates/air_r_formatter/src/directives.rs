use air_r_syntax::RSyntaxNode;
use air_r_syntax::{AnyRExpression, AnyRSelector, RCall, RIdentifier, RSyntaxKind};
use biome_formatter::CstFormatContext;
use biome_rowan::Language;
use biome_rowan::SyntaxNode;
use biome_rowan::SyntaxResult;
use comments::{Directive, FormatDirective};
use settings::Skip;
use settings::Table;

use crate::RFormatter;

/// Generic trait for extracting [comments::Directive]s from a node's comments
pub trait CommentDirectives {
    type Language: Language;

    /// Returns an iterator over [comments::Directive]s present in a node's comments
    fn directives(&self, node: &SyntaxNode<Self::Language>) -> impl Iterator<Item = Directive>;
}

/// Returns `true` if the node has a suppression comment and should use the same
/// formatting as in the source document.
///
/// Calls [biome_formatter::comments::Comments::mark_suppression_checked] on `node`.
#[inline]
pub(crate) fn has_skip_comment(node: &RSyntaxNode, f: &RFormatter) -> bool {
    // TODO: Weird to have a side effect in a predicate. Is this in the right
    // spot or should it be one level higher?
    f.context().comments().mark_suppression_checked(node);

    if !can_have_directive(node) {
        return false;
    }

    // Skip directives have precedence over all others
    f.comments()
        .directives(node)
        .any(|d| matches!(d, Directive::Format(FormatDirective::Skip)))
}

#[inline]
pub(crate) fn has_table_comment(node: &RSyntaxNode, f: &RFormatter) -> bool {
    if !can_have_directive(node) {
        return false;
    }

    f.comments()
        .directives(node)
        .any(|d| matches!(d, Directive::Format(FormatDirective::Table)))
}

#[inline]
fn can_have_directive(node: &RSyntaxNode) -> bool {
    let Some(parent) = node.parent() else {
        return false;
    };

    // Only expressions found within the following can take directives:
    // - Expression lists (program and braced blocks)
    // - Argument lists (like skipping an individual argument)
    // - Binary expression sides (like the RHS of a pipe chain)
    matches!(
        parent.kind(),
        RSyntaxKind::R_EXPRESSION_LIST
            | RSyntaxKind::R_ARGUMENT_LIST
            | RSyntaxKind::R_BINARY_EXPRESSION
    )
}

#[inline]
pub(crate) fn in_skip_setting(node: &RCall, f: &RFormatter) -> SyntaxResult<bool> {
    fn pred(node: RIdentifier, skip: &Skip) -> SyntaxResult<bool> {
        let node = node.name_token()?;
        Ok(skip.contains(node.text_trimmed()))
    }

    in_setting(node, f.options().skip(), pred)
}

#[inline]
pub(crate) fn in_table_setting(node: &RCall, f: &RFormatter) -> SyntaxResult<bool> {
    fn pred(node: RIdentifier, table: &Table) -> SyntaxResult<bool> {
        let node = node.name_token()?;
        Ok(table.contains(node.text_trimmed()))
    }

    in_setting(node, f.options().table(), pred)
}

fn in_setting<T, F>(node: &RCall, options: Option<T>, pred: F) -> SyntaxResult<bool>
where
    F: Fn(RIdentifier, T) -> SyntaxResult<bool>,
{
    let Some(options) = options else {
        return Ok(false);
    };

    Ok(match node.function()? {
        AnyRExpression::RIdentifier(node) => pred(node, options)?,
        AnyRExpression::RNamespaceExpression(node) => match node.right()? {
            AnyRSelector::RIdentifier(node) => pred(node, options)?,
            _ => false,
        },
        _ => false,
    })
}
