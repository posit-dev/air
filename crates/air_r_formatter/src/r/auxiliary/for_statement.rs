use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RForStatement;
use air_r_syntax::RForStatementFields;
use biome_formatter::format_args;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRForStatement;
impl FormatNodeRule<RForStatement> for FormatRForStatement {
    fn fmt_fields(&self, node: &RForStatement, f: &mut RFormatter) -> FormatResult<()> {
        let RForStatementFields {
            for_token,
            l_paren_token,
            variable,
            in_token,
            sequence,
            r_paren_token,
            body,
        } = node.as_fields();

        write!(
            f,
            [group(&format_args!(
                for_token.format(),
                space(),
                l_paren_token.format(),
                variable.format(),
                space(),
                in_token.format(),
                space(),
                sequence.format(),
                r_paren_token.format(),
                FormatForStatementBody::new(&body?)
            ))]
        )
    }
}

pub(crate) struct FormatForStatementBody<'a> {
    body: &'a AnyRExpression,
}

impl<'a> FormatForStatementBody<'a> {
    pub fn new(body: &'a AnyRExpression) -> Self {
        Self { body }
    }
}

impl Format<RFormatContext> for FormatForStatementBody<'_> {
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

        write!(f, [soft_line_indent_or_space(&self.body.format())])
    }
}
