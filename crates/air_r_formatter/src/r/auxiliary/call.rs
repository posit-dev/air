use crate::comments_directives;
use crate::is_suppressed_by_comment;
use crate::prelude::*;
use crate::r::auxiliary::call_arguments::FormatRCallArgumentsOptions;

use air_r_syntax::AnyRExpression;
use air_r_syntax::AnyRSelector;
use air_r_syntax::RCall;
use air_r_syntax::RCallFields;
use air_r_syntax::RIdentifier;
use air_r_syntax::RLanguage;
use biome_formatter::FormatRuleWithOptions;
use biome_formatter::write;
use biome_rowan::AstNode;
use biome_rowan::SyntaxResult;
use comments::Directive;
use comments::FormatDirective;
use settings::Skip;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRCall {
    table: Option<bool>,
}

impl FormatNodeRule<RCall> for FormatRCall {
    fn fmt_fields(&self, node: &RCall, f: &mut RFormatter) -> FormatResult<()> {
        let RCallFields {
            function,
            arguments,
        } = node.as_fields();

        let table = self.table.unwrap_or_else(|| is_table_call(node, f));
        let options = FormatRCallArgumentsOptions { table };

        write!(
            f,
            [function.format(), arguments.format()?.with_options(options)]
        )
    }

    fn is_suppressed(&self, node: &RCall, f: &RFormatter) -> bool {
        is_suppressed_by_comment(node, f) || is_skip_by_option(node, f).unwrap_or(false)
    }
}

impl FormatRuleWithOptions<RCall> for FormatRCall {
    type Options = FormatRCallArgumentsOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.table = Some(options.table);
        self
    }
}

#[inline]
fn is_skip_by_option(node: &RCall, f: &RFormatter) -> SyntaxResult<bool> {
    fn is_skip(node: RIdentifier, skip: &Skip) -> SyntaxResult<bool> {
        let node = node.name_token()?;
        Ok(skip.contains(node.text_trimmed()))
    }

    is_match_by_option(node, f.options().skip(), is_skip)
}

pub(crate) fn is_table_call(node: &RCall, f: &RFormatter) -> bool {
    is_table_by_option(node, f).unwrap_or(false) || is_table_by_comment(node, f)
}

fn is_table_by_option(node: &RCall, f: &RFormatter) -> SyntaxResult<bool> {
    fn is_table(node: RIdentifier, table: &settings::Table) -> SyntaxResult<bool> {
        let node = node.name_token()?;
        Ok(table.contains(node.text_trimmed()))
    }

    is_match_by_option(node, f.options().table(), is_table)
}

// Generic and used by RCall and RBinaryExpression
pub(crate) fn is_table_by_comment<N>(node: &N, f: &RFormatter) -> bool
where
    N: AstNode<Language = RLanguage>,
{
    comments_directives(node, f)
        .into_iter()
        .any(|d| matches!(d, Directive::Format(FormatDirective::Table)))
}

fn is_match_by_option<T, F>(node: &RCall, options: Option<T>, pred: F) -> SyntaxResult<bool>
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
