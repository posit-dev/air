use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use biome_formatter::separated::TrailingSeparator;
use air_r_syntax::RParameterList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRParameterList;
impl FormatRule<RParameterList> for FormatRParameterList {
    type Context = RFormatContext;
    fn fmt(&self, node: &RParameterList, f: &mut RFormatter) -> FormatResult<()> {
        let mut joiner = f.join_nodes_with_soft_line();
        join_parameter_list(&mut joiner, &node)?;
        joiner.finish()
    }
}

fn join_parameter_list<S>(
    joiner: &mut JoinNodesBuilder<'_, '_, S, RFormatContext>,
    list: &RParameterList,
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
