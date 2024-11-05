use crate::prelude::*;
use crate::separated::FormatAstSeparatedListExtension;
use air_r_syntax::AnyRArgument;
use air_r_syntax::RArgumentList;
use air_r_syntax::RSyntaxKind;
use biome_formatter::separated::TrailingSeparator;
use biome_formatter::write;
use biome_rowan::AstSeparatedList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRArgumentList;
impl FormatRule<RArgumentList> for FormatRArgumentList {
    type Context = RFormatContext;
    fn fmt(&self, node: &RArgumentList, f: &mut RFormatter) -> FormatResult<()> {
        // TODO: Special handling for comments? See `handle_array_holes` for JS.

        if should_join_arguments_with_space(node)? {
            let formattable = format_with(|f| join_arguments_with_space(node, f));
            write!(f, [&formattable])
        } else {
            let formattable = format_with(|f| join_arguments_with_soft_line(node, f));
            write!(f, [group(&soft_block_indent(&formattable))])
        }
    }
}

/// Check for a special case consisting of a function call where:
///
/// - The last argument is a braced expression
/// - None of the other arguments break across multiple lines
///
/// In this case, we join all arguments with a space, but that's it. The
/// trailing braced expression will nicely expand over multiple lines as it
/// formats itself.
///
/// ```r
/// test_that("description", {
///   1 + 1
/// })
///
/// with(data, {
///   col1 + col2
/// })
/// ```
///
/// Forcibly joining with a space means that extremely long argument lists that
/// end with a braced expression can exceed the line limit, but this is useful
/// for very long testthat descriptions.
///
/// ```r
/// test_that("a very long and detailed description about the problem that exceeds the limit", {
///   1 + 1
/// })
/// ```
fn should_join_arguments_with_space(node: &RArgumentList) -> FormatResult<bool> {
    use AnyRArgument::*;
    use RSyntaxKind::*;

    for node in node.iter() {
        if node?.syntax().has_leading_newline() {
            return Ok(false);
        }
    }

    let Some(last) = node.last() else {
        return Ok(false);
    };

    // Check if the last argument is a braced expression
    Ok(match last? {
        RNamedArgument(node) => match node.value() {
            None => false,
            Some(node) => node.syntax().kind() == R_BRACED_EXPRESSIONS,
        },
        RUnnamedArgument(node) => node.value()?.syntax().kind() == R_BRACED_EXPRESSIONS,
        _ => false,
    })
}

fn join_arguments_with_space(node: &RArgumentList, f: &mut RFormatter) -> FormatResult<()> {
    let mut joiner = f.join_nodes_with_space();
    join_argument_list(&mut joiner, node)?;
    joiner.finish()
}

fn join_arguments_with_soft_line(node: &RArgumentList, f: &mut RFormatter) -> FormatResult<()> {
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
