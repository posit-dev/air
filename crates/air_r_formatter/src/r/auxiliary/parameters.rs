use crate::context::RFormatOptions;
use crate::prelude::*;
use air_r_syntax::RParameterList;
use air_r_syntax::RParameters;
use air_r_syntax::RParametersFields;
use biome_formatter::format_args;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRParameters;
impl FormatNodeRule<RParameters> for FormatRParameters {
    fn fmt_fields(&self, node: &RParameters, f: &mut RFormatter) -> FormatResult<()> {
        let RParametersFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        // Special case where the dangling comment has no node to attach to:
        //
        // ```r
        // function(
        //   # dangling comment
        // ) {}
        // ```
        //
        // If we don't handle it specially, it can break idempotence.
        // Same as `RCallLikeArguments`.
        if items.is_empty() {
            return write!(
                f,
                [
                    l_paren_token.format(),
                    format_dangling_comments(node.syntax()).with_soft_block_indent(),
                    r_paren_token.format()
                ]
            );
        }

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&items.format()),
                r_paren_token.format()
            ])
            .should_expand(has_persistent_line_break(&items, f.options()))]
        )
    }

    fn fmt_dangling_comments(&self, _: &RParameters, _: &mut RFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        // Only applicable for the empty arguments case
        Ok(())
    }
}

/// Check if the user has inserted a persistent line break before the very first `parameter`.
/// If so, we respect that and treat it as a request to break ALL of the parameters in
/// this function definition. Note this is a case of irreversible formatting!
///
/// ```r
/// # Fits on one line, but newline before `a` forces ALL parameters to break
///
/// # Input
/// fn <- function(
/// a, b, c) {
///   body
/// }
///
/// # Output
/// fn <- function(
///   a,
///   b,
///   c
/// ) {
///   body
/// }
/// ```
///
/// Note that removing this line break is a request to flatten if possible. By only having
/// this special behavior on the very first parameter, we make it easy to request flattening.
///
/// ```r
/// # Say we start here and want to flatten
/// fn <- function(
///   a,
///   b,
///   c
/// ) {
///   body
/// }
///
/// # Remove the first line break and run air
/// fn <- function(a,
///   b,
///   c
/// ) {
///   body
/// }
///
/// # Output
/// fn <- function(a, b, c) {
///   body
/// }
/// ```
fn has_persistent_line_break(items: &RParameterList, options: &RFormatOptions) -> bool {
    if options.persistent_line_breaks().is_ignore() {
        return false;
    }

    // Dig down to the first parameter
    // (Could be an empty parameter list, and first element could be a syntax error)
    let Some(first) = items.first() else {
        return false;
    };
    let Ok(first) = first else {
        return false;
    };

    first.syntax().has_leading_newline()
}
