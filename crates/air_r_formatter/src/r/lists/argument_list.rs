use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use air_r_syntax::RArgumentList;
use biome_formatter::separated::TrailingSeparator;
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRArgumentList;
impl FormatRule<RArgumentList> for FormatRArgumentList {
    type Context = RFormatContext;
    fn fmt(&self, node: &RArgumentList, f: &mut RFormatter) -> FormatResult<()> {
        // TODO: Special handling for comments? See `handle_array_holes` for JS.
        let mut joiner = f.join_nodes_with_soft_line();
        join_argument_list(&mut joiner, node)?;
        joiner.finish()
    }
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
