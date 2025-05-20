use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use biome_formatter::write;

pub(crate) struct FormatStatementBody<'a> {
    body: &'a AnyRExpression,
}

impl<'a> FormatStatementBody<'a> {
    pub fn new(body: &'a AnyRExpression) -> Self {
        Self { body }
    }
}

// TODO!: Repurpose this as `FormatBracedBody` for use in:
// - For loops (unconditionally)
// - Repeat loops (unconditionally)
// - While loops (unconditionally)
// - Function definitions
//   - Includes anonymous functions
//   - Allow 1 liner function definitions
//   - Definitely breaks if argument list expands over multiple lines
//   - Use `if_group_breaks(&text("{"))` to add missing `{}` if the group breaks,
//     like if statements. Probably won't be able to use simple `FormatBracedBody`
impl Format<RFormatContext> for FormatStatementBody<'_> {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        use AnyRExpression::*;

        // We only want a single space between the construct and the statement
        // body if the body is a braced expression. The expression will handle
        // the line break as required. Otherwise, we handle the soft line
        // indent or space for all other syntax.
        //
        // for (x in xs) {}
        //              |--| statement body
        //
        // for (x in xs)
        //   a
        //  |--| statement body
        if matches!(&self.body, RBracedExpressions(_)) {
            write!(f, [space(), self.body.format()])
        } else {
            write!(f, [soft_line_indent_or_space(&self.body.format())])
        }
    }
}
