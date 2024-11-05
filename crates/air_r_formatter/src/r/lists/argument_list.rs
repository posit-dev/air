use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use air_r_syntax::{RArgumentList, RBracedExpressions};
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::write;
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRArgumentList;
impl FormatRule<RArgumentList> for FormatRArgumentList {
    type Context = RFormatContext;
    fn fmt(&self, node: &RArgumentList, f: &mut RFormatter) -> FormatResult<()> {
        // TODO: Special handling for comments? See `handle_array_holes` for JS.

        let last_argument_is_braced_expression = node.last().is_some_and(|n| {
            n.is_ok_and(|n| {
                let Some(arg) = n.as_r_unnamed_argument() else {
                    return false;
                };
                let Ok(arg) = arg.value() else {
                    return false;
                };
                RBracedExpressions::can_cast(arg.syntax().kind())
            })
        });

        let has_leading_newlines = node
            .iter()
            .any(|n| n.is_ok_and(|n| n.syntax().has_leading_newline()));

        if !has_leading_newlines && last_argument_is_braced_expression {
            let formattable = format_with(|f| no_expand_argument_list(node, f));
            write!(f, [&formattable])
        } else {
            let formattable = format_with(|f| fully_expand_argument_list(node, f));
            write!(f, [group(&soft_block_indent(&formattable))])
        }
    }
}

fn no_expand_argument_list(node: &RArgumentList, f: &mut RFormatter) -> FormatResult<()> {
    let mut joiner = f.join_nodes_with_space();
    join_argument_list(&mut joiner, node)?;
    joiner.finish()
}

fn fully_expand_argument_list(node: &RArgumentList, f: &mut RFormatter) -> FormatResult<()> {
    let mut joiner = f.join_nodes_with_soft_line();
    join_argument_list(&mut joiner, node)?;
    joiner.finish()
}

fn join_argument_list<S>(
    joiner: &mut JoinNodesBuilder<'_, '_, S, RFormatContext>,
    list: &RArgumentList,
) -> FormatResult<()>
where
    S: Format<RFormatContext>,
{
    let entries = list
        .format_separated(",")
        .with_trailing_separator(TrailingSeparator::Disallowed)
        .zip(list.iter());

    for (format_entry, node) in entries {
        joiner.entry(node?.syntax(), &format_entry);
    }

    Ok(())
}
