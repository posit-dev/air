use crate::prelude::*;
use crate::r::auxiliary::call_arguments::RCallLikeArguments;
use air_r_syntax::RSubsetArguments;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRSubsetArguments;
impl FormatNodeRule<RSubsetArguments> for FormatRSubsetArguments {
    fn fmt_fields(&self, node: &RSubsetArguments, f: &mut RFormatter) -> FormatResult<()> {
        RCallLikeArguments::Subset(node.clone()).fmt(f)
    }

    fn fmt_dangling_comments(&self, _: &RSubsetArguments, _: &mut RFormatter) -> FormatResult<()> {
        // Formatted inside of `fmt_fields`
        // Only applicable for the empty arguments case
        Ok(())
    }
}
