use crate::{RFormatter, comments_directives};
use air_r_syntax::{AnyRExpression, AnyRSelector, RCall, RIdentifier, RLanguage, RSyntaxKind};
use biome_formatter::CstFormatContext;
use biome_rowan::{AstNode, SyntaxResult};
use comments::{Directive, FormatDirective};
use settings::Skip;

/// Returns `true` if the node has a suppression comment and should use the same formatting as in the source document.
///
/// Calls [biome_formatter::comments::Comments::mark_suppression_checked] on `node`.
#[inline]
pub(crate) fn has_skip_comment<N>(node: &N, f: &RFormatter) -> bool
where
    N: AstNode<Language = RLanguage>,
{
    // TODO: Weird to have a side effect in a predicate. Is this in the right
    // spot or should it be one level higher?
    f.context()
        .comments()
        .mark_suppression_checked(node.syntax());

    if !can_have_directive(node) {
        return false;
    }

    // Skip directives have precedence over all others
    comments_directives(node, f).any(|d| matches!(d, Directive::Format(FormatDirective::Skip)))
}

// Generic and used by RCall and RBinaryExpression
#[inline]
pub(crate) fn has_table_comment<N>(node: &N, f: &RFormatter) -> bool
where
    N: AstNode<Language = RLanguage>,
{
    if !can_have_directive(node) {
        return false;
    }

    comments_directives(node, f).any(|d| matches!(d, Directive::Format(FormatDirective::Table)))
}

#[inline]
fn can_have_directive<N>(node: &N) -> bool
where
    N: AstNode<Language = RLanguage>,
{
    let Some(parent) = node.syntax().parent() else {
        return false;
    };

    // Only expression lists (program and braced blocks) and argument lists can be suppressed
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
    fn pred(node: RIdentifier, table: &settings::Table) -> SyntaxResult<bool> {
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
