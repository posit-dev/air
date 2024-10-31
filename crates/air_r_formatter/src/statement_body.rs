use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use biome_formatter::write;

pub(crate) struct FormatStatementBody<'a> {
    body: &'a AnyRExpression,
    force_space: bool,
}

impl<'a> FormatStatementBody<'a> {
    pub fn new(body: &'a AnyRExpression) -> Self {
        Self {
            body,
            force_space: false,
        }
    }

    /// For `if () 1 else if () 2` scenarios, ensures the second `if` is started
    /// on the same line as the `else` (rather than line broken) and is
    /// separated from the `else` by a single space
    pub fn with_forced_space(mut self, value: bool) -> Self {
        self.force_space = value;
        self
    }
}

impl Format<RFormatContext> for FormatStatementBody<'_> {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        use AnyRExpression::*;

        // We only want a single space between the construct and the statement
        // body if the body is a braced expression. The expression will handle
        // the line break as required. Otherwise, we handle the soft line
        // indent for all other syntax.
        //
        // if (a) {}
        //       |--| statement body
        //
        // if (a)
        //   a
        //  |--| statement body
        if matches!(&self.body, RBracedExpressions(_)) || self.force_space {
            write!(f, [space(), self.body.format()])
        } else {
            write!(f, [soft_line_indent_or_space(&self.body.format())])
        }
    }
}
