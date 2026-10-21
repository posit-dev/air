use air_r_syntax::RLanguage;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::RSyntaxNode;
use air_r_syntax::{AnyRExpression, AnyRSelector, RCall, RIdentifier};
use biome_rowan::Language;
use biome_rowan::SyntaxNode;
use biome_rowan::SyntaxResult;
use comments::{Directive, FormatDirective};
use settings::Skip;
use settings::Table;

use crate::RFormatter;
use crate::comments::RComments;

/// Extension trait for [biome_formatter::Comments]
///
/// Currently this is for extending [biome_formatter::Comments] with knowledge about our
/// comment directives
pub trait CommentsExt {
    type Language: Language;

    /// Does this node contain a `# fmt: skip` directive?
    fn has_skip_directive(&self, node: &SyntaxNode<Self::Language>) -> bool;

    /// Does this node contain a `# fmt: table` directive?
    fn has_table_directive(&self, node: &SyntaxNode<Self::Language>) -> bool;
}

impl CommentsExt for RComments {
    type Language = RLanguage;

    fn has_skip_directive(&self, node: &SyntaxNode<Self::Language>) -> bool {
        directives(self, node).any(|d| matches!(d, Directive::Format(FormatDirective::Skip)))
    }

    fn has_table_directive(&self, node: &SyntaxNode<Self::Language>) -> bool {
        directives(self, node).any(|d| matches!(d, Directive::Format(FormatDirective::Table)))
    }
}

fn directives(comments: &RComments, node: &RSyntaxNode) -> impl Iterator<Item = Directive> {
    // We intentionally only consider directives in leading comments. This is a
    // departure from Biome (and Ruff?).
    let leading = match directive_node(node) {
        Some(node) => comments.leading_comments(&node),
        None => &[],
    };

    leading
        .iter()
        .filter_map(|c| comments::parse_comment_directive(c.piece().text()))
}

/// The node whose leading comments hold the directives for `node`, if any
///
/// Only nodes found within the following can take directives:
/// - Statement lists (program and braced blocks)
/// - Argument lists (like skipping an individual argument)
/// - Binary expression sides (like the RHS of a pipe chain)
fn directive_node(node: &RSyntaxNode) -> Option<RSyntaxNode> {
    let parent = node.parent()?;

    match parent.kind() {
        RSyntaxKind::R_STATEMENT_LIST => {
            // i.e. a statement at top-level or within braced statements. Note
            // that the statement wraps the expression plus its optional
            // semicolon token.
            //
            // ```r
            // # fmt: skip
            // 1+1
            //
            // # fmt: skip
            // 1+1; # Keeps the `;`
            //
            // {
            //   # fmt: skip
            //   1+1
            // }
            // ```
            Some(node.clone())
        }
        RSyntaxKind::R_BINARY_EXPRESSION => {
            // i.e. the RHS of a pipe chain
            //
            // ```r
            // df |>
            //   # fmt: skip
            //   mutate(x=1)
            // ```
            Some(node.clone())
        }
        RSyntaxKind::R_ARGUMENT_LIST => {
            // i.e. skipping an individual argument
            //
            // ```r
            // fn(
            //   # fmt: skip
            //   1+1,
            //   2 + 2
            // )
            // ```
            Some(node.clone())
        }
        RSyntaxKind::R_EXPRESSION_STATEMENT => {
            // Most of the time, we check directives from outer to inner and
            // hit:
            // - `node = RExpressionStatement`
            // - `parent = RStatementList` which sends us through the
            // `R_STATEMENT_LIST` branch above. This is the case for suppression
            // directives.
            //
            // But with `# fmt: table` we are already holding the `RCall` or
            // `RBinaryExpression` of interest and want its directives. In that
            // case, the parent is `RExpressionStatement`, and all leading
            // comments will have been placed on that parent
            // `RExpressionStatement` instead since it shares a start location
            // with the `RCall` / `RBinaryExpression`, so the
            // `RExpressionStatement` is the directive node of interest and we
            // return `parent` from here.
            //
            // i.e. we end up here due to how this is handled in `FormatRCall`:
            //
            // ```r
            // # fmt: table
            // tribble(
            //   ~x, ~y,
            //   1, 2
            // )
            // ```
            Some(parent)
        }
        _ => None,
    }
}

/// Is the name of this function call contained within the `skip` `air.toml` setting?
pub(crate) fn in_skip_setting(node: &RCall, f: &RFormatter) -> SyntaxResult<bool> {
    fn pred(node: RIdentifier, skip: &Skip) -> SyntaxResult<bool> {
        let node = node.name_token()?;
        Ok(skip.contains(node.text_trimmed()))
    }

    in_setting(node, f.options().skip(), pred)
}

/// Is the name of this function call contained within the `table` `air.toml` setting?
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
