use biome_formatter::{prelude::*, write, Format, FormatResult};
use biome_rowan::{Language, SyntaxNode};

// For working generically with joiners
pub trait Joiner<'fmt, 'buf, Separator, Context>
where
    Separator: Format<Context>,
{
    fn entry<L: Language>(&mut self, node: &SyntaxNode<L>, content: &dyn Format<Context>);
    fn entry_no_separator(&mut self, content: &dyn Format<Context>);
    fn entries<L, F, I>(&mut self, entries: I) -> &mut Self
    where
        L: Language,
        F: Format<Context>,
        I: IntoIterator<Item = (SyntaxNode<L>, F)>;

    fn finish(&mut self) -> FormatResult<()>;
}

// Implement the trait for Biome's joiner
impl<'fmt, 'buf, Separator, Context> Joiner<'fmt, 'buf, Separator, Context>
    for JoinNodesBuilder<'fmt, 'buf, Separator, Context>
where
    Separator: Format<Context>,
{
    fn entry<L: Language>(&mut self, node: &SyntaxNode<L>, content: &dyn Format<Context>) {
        JoinNodesBuilder::entry(self, node, content)
    }

    fn entry_no_separator(&mut self, content: &dyn Format<Context>) {
        JoinNodesBuilder::entry_no_separator(self, content)
    }

    fn entries<L, F, I>(&mut self, entries: I) -> &mut Self
    where
        L: Language,
        F: Format<Context>,
        I: IntoIterator<Item = (SyntaxNode<L>, F)>,
    {
        JoinNodesBuilder::entries(self, entries)
    }

    fn finish(&mut self) -> FormatResult<()> {
        JoinNodesBuilder::finish(self)
    }
}

/// Version of `JoinNodesBuilder` that respects at maximum 2 lines between inputs
/// From https://github.com/biomejs/biome/blob/main/crates/biome_formatter/src/comments/builder.rs
#[must_use = "must eventually call `finish()` on Format builders"]
pub struct StretchyJoinNodesBuilder<'fmt, 'buf, Separator, Context> {
    result: FormatResult<()>,
    /// The separator to insert between nodes. Either a soft or hard line break
    separator: Separator,
    fmt: &'fmt mut Formatter<'buf, Context>,
    has_elements: bool,
}

impl<'fmt, 'buf, Separator, Context> StretchyJoinNodesBuilder<'fmt, 'buf, Separator, Context> {
    pub(crate) fn new(separator: Separator, fmt: &'fmt mut Formatter<'buf, Context>) -> Self {
        Self {
            result: Ok(()),
            separator,
            fmt,
            has_elements: false,
        }
    }
}

impl<'fmt, 'buf, Separator, Context> Joiner<'fmt, 'buf, Separator, Context>
    for StretchyJoinNodesBuilder<'fmt, 'buf, Separator, Context>
where
    Separator: Format<Context>,
{
    /// Adds a new node with the specified formatted content to the output, respecting any new lines
    /// that appear before the node in the input source.
    fn entry<L: Language>(&mut self, node: &SyntaxNode<L>, content: &dyn Format<Context>) {
        self.result = self.result.and_then(|_| {
            if self.has_elements {
                let n_lines = get_lines_before(node);
                if n_lines >= 2 {
                    // AIR: This branch is the main difference with the upstream variant
                    write!(self.fmt, [empty_line(), empty_line()])?;
                } else if n_lines == 1 {
                    write!(self.fmt, [empty_line()])?;
                } else {
                    self.separator.fmt(self.fmt)?;
                }
            }

            self.has_elements = true;

            write!(self.fmt, [content])
        });
    }

    /// Writes an entry without adding a separating line break or empty line.
    fn entry_no_separator(&mut self, content: &dyn Format<Context>) {
        self.result = self.result.and_then(|_| {
            self.has_elements = true;

            write!(self.fmt, [content])
        })
    }

    /// Adds an iterator of entries to the output. Each entry is a `(node, content)` tuple.
    fn entries<L, F, I>(&mut self, entries: I) -> &mut Self
    where
        L: Language,
        F: Format<Context>,
        I: IntoIterator<Item = (SyntaxNode<L>, F)>,
    {
        for (node, content) in entries {
            self.entry(&node, &content)
        }

        self
    }

    fn finish(&mut self) -> FormatResult<()> {
        self.result
    }
}
