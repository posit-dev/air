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

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&items.format()),
                r_paren_token.format()
            ])
            .should_expand(has_magic_line_break(&items, f.options()))]
        )
    }
}

/// Check if the user has inserted a magic line break before the very first `parameter`.
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
fn has_magic_line_break(items: &RParameterList, options: &RFormatOptions) -> bool {
    if options.magic_line_break().is_ignore() {
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
