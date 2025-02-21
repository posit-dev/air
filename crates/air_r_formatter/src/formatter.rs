use biome_formatter::prelude::{hard_line_break, Formatter, Line};

use crate::joiner::{EmptyLines, JoinNodesBuilderExt};

pub trait FormatterExt<'buf, Context> {
    /// Specialized version of `join_nodes_with_hardline()` that allows up to 2
    /// hard lines.
    fn join_nodes_with_hardline_ext<'a>(
        &'a mut self,
        empty_lines: EmptyLines,
    ) -> JoinNodesBuilderExt<'a, 'buf, Line, Context>;
}

impl<'buf, Context> FormatterExt<'buf, Context> for Formatter<'buf, Context> {
    fn join_nodes_with_hardline_ext<'a>(
        &'a mut self,
        empty_lines: EmptyLines,
    ) -> JoinNodesBuilderExt<'a, 'buf, Line, Context> {
        JoinNodesBuilderExt::new(hard_line_break(), empty_lines, self)
    }
}
