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
        // TODO:
        // For block statements that start with `{`, we want to try and keep
        // the `{` on the same line as the for loop `()`, and then let the
        // block statement format itself from there. Otherwise we forcibly
        // indent 1 liner for loops for clarity.
        //
        // use AnyRExpression::*;
        // if  let RBlockStatement() = &self.body {
        //     write!(f, [space(), self.body.format()])
        // } else {

        if self.force_space {
            write!(f, [space(), self.body.format()])
        } else {
            write!(f, [soft_line_indent_or_space(&self.body.format())])
        }
    }
}
