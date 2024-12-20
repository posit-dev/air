use crate::prelude::*;
use crate::r::auxiliary::call_arguments::RCallLikeArguments;
use air_r_syntax::RSubset2Arguments;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRSubset2Arguments;
impl FormatNodeRule<RSubset2Arguments> for FormatRSubset2Arguments {
    fn fmt_fields(&self, node: &RSubset2Arguments, f: &mut RFormatter) -> FormatResult<()> {
        RCallLikeArguments::Subset2(node.clone()).fmt(f)
    }

    fn fmt_dangling_comments(&self, _: &RSubset2Arguments, _: &mut RFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        // Only applicable for the empty arguments case
        Ok(())
    }
}
