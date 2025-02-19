use biome_formatter::prelude::{hard_line_break, Formatter, Line};

use crate::joiner::StretchyJoinNodesBuilder;

pub trait FormatterExt<'buf, Context> {
    /// Specialized version of `join_nodes_with_hardline()` that allows up to 2 hard lines.
    fn join_nodes_with_stretchy_hardline<'a>(
        &'a mut self,
    ) -> StretchyJoinNodesBuilder<'a, 'buf, Line, Context>;
}

impl<'buf, Context> FormatterExt<'buf, Context> for Formatter<'buf, Context> {
    fn join_nodes_with_stretchy_hardline<'a>(
        &'a mut self,
    ) -> StretchyJoinNodesBuilder<'a, 'buf, Line, Context> {
        StretchyJoinNodesBuilder::new(hard_line_break(), self)
    }
}
