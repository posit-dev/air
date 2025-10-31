use crate::prelude::*;
use air_r_syntax::RArgument;
use air_r_syntax::RArgumentFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRArgument;
impl FormatNodeRule<RArgument> for FormatRArgument {
    fn fmt_fields(&self, node: &RArgument, f: &mut RFormatter) -> FormatResult<()> {
        fmt_argument_fields(node, f)
    }
}

pub(crate) fn fmt_argument_fields(node: &RArgument, f: &mut RFormatter) -> FormatResult<()> {
    let RArgumentFields { name_clause, value } = node.as_fields();

    match (name_clause, value) {
        // Hole
        // `foo(,)`
        // `foo(value, )`
        // `foo(, value)`
        (None, None) => Ok(()),

        // Unnamed argument
        // `foo(value)`
        // `foo(value, value)`
        (None, Some(value)) => write!(f, [value.format()]),

        // Named argument without a value
        // We write a mandatory space as a signal that this is a fairly
        // weird a nonstandard thing to see.
        // `foo(name = )`
        // `foo(name = , value)`
        (Some(name_clause), None) => write!(f, [name_clause.format(), space()]),

        // Named argument with a value
        // `foo(name = value)`
        (Some(name_clause), Some(value)) => {
            // If `value` has leading comments (which might happen due to our
            // comment placement rules in arguments), we consistently break
            // after the name clause and indent the value
            if f.comments().has_leading_comments(value.syntax()) {
                write!(
                    f,
                    [
                        name_clause.format(),
                        space(),
                        // Note that this _will_ break, since the comment causes a hard line break
                        soft_line_indent_or_space(&value.format())
                    ]
                )
            } else {
                // Never break before the value, even if `name = value` doesn't
                // fit on one line. This allows more consistent formatting, but
                // the main reason is for behaviour within a group. A soft line
                // break would always break within expanded calls, even if the
                // name-value pair comfortably fits on a line.
                write!(f, [name_clause.format(), space(), value.format()])
            }
        }
    }
}
