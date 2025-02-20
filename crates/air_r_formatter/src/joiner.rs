use biome_formatter::{prelude::*, write, Format, FormatResult};
use biome_rowan::{Language, SyntaxNode};

/// Version of `JoinNodesBuilder` that can be configured to respect maximum n lines between inputs.
/// From https://github.com/biomejs/biome/blob/main/crates/biome_formatter/src/comments/builder.rs
#[must_use = "must eventually call `finish()` on Format builders"]
pub struct StretchyJoinNodesBuilder<'fmt, 'buf, Separator, Context> {
    result: FormatResult<()>,
    /// The separator to insert between nodes. Either a soft or hard line break
    separator: Separator,
    fmt: &'fmt mut Formatter<'buf, Context>,
    has_elements: bool,
    stretchy: bool,
}

impl<'fmt, 'buf, Separator, Context> StretchyJoinNodesBuilder<'fmt, 'buf, Separator, Context> {
    pub(crate) fn new(
        separator: Separator,
        stretchy: bool,
        fmt: &'fmt mut Formatter<'buf, Context>,
    ) -> Self {
        Self {
            result: Ok(()),
            separator,
            fmt,
            has_elements: false,
            stretchy,
        }
    }
}

impl<Separator, Context> StretchyJoinNodesBuilder<'_, '_, Separator, Context>
where
    Separator: Format<Context>,
{
    /// Adds a new node with the specified formatted content to the output, respecting any new lines
    /// that appear before the node in the input source.
    pub fn entry<L: Language>(&mut self, node: &SyntaxNode<L>, content: &dyn Format<Context>) {
        self.result = self.result.and_then(|_| {
            if self.has_elements {
                let n_lines = get_lines_before(node);
                if self.stretchy && n_lines > 2 {
                    // AIR: This branch is the main difference with the upstream variant
                    write!(self.fmt, [empty_line(), text("\n")])?;
                } else if n_lines > 1 {
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
    pub fn entry_no_separator(&mut self, content: &dyn Format<Context>) {
        self.result = self.result.and_then(|_| {
            self.has_elements = true;

            write!(self.fmt, [content])
        })
    }

    /// Adds an iterator of entries to the output. Each entry is a `(node, content)` tuple.
    pub fn entries<L, F, I>(&mut self, entries: I) -> &mut Self
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

    pub fn finish(&mut self) -> FormatResult<()> {
        self.result
    }
}
