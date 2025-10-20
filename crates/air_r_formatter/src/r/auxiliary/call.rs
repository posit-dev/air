use crate::directives::CommentDirectives;
use crate::directives::in_skip_setting;
use crate::directives::in_table_setting;
use crate::prelude::*;
use crate::r::auxiliary::call_arguments::FormatRCallArgumentsOptions;

use air_r_syntax::RCall;
use air_r_syntax::RCallFields;
use biome_formatter::FormatRuleWithOptions;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRCall {
    table: Option<bool>,
}

impl FormatNodeRule<RCall> for FormatRCall {
    fn fmt_fields(&self, node: &RCall, f: &mut RFormatter) -> FormatResult<()> {
        let RCallFields {
            function,
            arguments,
        } = node.as_fields();

        let table = self.table.unwrap_or_else(|| is_table(node, f));
        let options = FormatRCallArgumentsOptions { table };

        write!(
            f,
            [function.format(), arguments.format()?.with_options(options)]
        )
    }

    fn is_suppressed(&self, node: &RCall, f: &RFormatter) -> bool {
        let comments = f.comments();
        comments.mark_suppression_checked(node.syntax());
        comments.has_skip_directive(node.syntax()) || in_skip_setting(node, f).unwrap_or(false)
    }
}

fn is_table(node: &RCall, f: &RFormatter) -> bool {
    let comments = f.comments();
    comments.has_table_directive(node.syntax()) || in_table_setting(node, f).unwrap_or(false)
}

impl FormatRuleWithOptions<RCall> for FormatRCall {
    type Options = FormatRCallArgumentsOptions;

    fn with_options(mut self, options: Self::Options) -> Self {
        self.table = Some(options.table);
        self
    }
}
