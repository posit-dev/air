use crate::prelude::*;
use air_r_syntax::RCallArguments;
use air_r_syntax::RCallArgumentsFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRCallArguments;
impl FormatNodeRule<RCallArguments> for FormatRCallArguments {
    fn fmt_fields(&self, node: &RCallArguments, f: &mut RFormatter) -> FormatResult<()> {
        let RCallArgumentsFields {
            l_paren_token,
            items,
            r_paren_token,
        } = node.as_fields();

        // TODO:
        // Look at `write_grouped_arguments()` on the JS side to figure out how
        // to nicely format things like this, where the last node is a block
        // expression:
        //
        // ```r
        // test_that("description", {
        //   1 + 1
        // })
        // ```
        write!(
            f,
            [
                l_paren_token.format(),
                &items.format(),
                r_paren_token.format()
            ]
        )
    }
}
