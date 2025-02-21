use biome_formatter::{prelude::*, write, Format, FormatResult};
use biome_rowan::{Language, SyntaxNode};

/// How many lines are allowed between elements
#[derive(Debug, Clone, Copy, Default)]
pub enum EmptyLines {
    #[default]
    Single,
    Double,
}

/// Version of `JoinNodesBuilder` that can be configured to respect maximum n lines between inputs.
/// From https://github.com/biomejs/biome/blob/main/crates/biome_formatter/src/comments/builder.rs
#[must_use = "must eventually call `finish()` on Format builders"]
pub struct AirJoinNodesBuilder<'fmt, 'buf, Separator, Context> {
    result: FormatResult<()>,
    /// The separator to insert between nodes. Either a soft or hard line break
    separator: Separator,
    fmt: &'fmt mut Formatter<'buf, Context>,
    has_elements: bool,
    empty_lines: EmptyLines,
}

impl<'fmt, 'buf, Separator, Context> AirJoinNodesBuilder<'fmt, 'buf, Separator, Context> {
    pub(crate) fn new(
        separator: Separator,
        empty_lines: EmptyLines,
        fmt: &'fmt mut Formatter<'buf, Context>,
    ) -> Self {
        Self {
            result: Ok(()),
            separator,
            fmt,
            has_elements: false,
            empty_lines,
        }
    }
}

impl<Separator, Context> AirJoinNodesBuilder<'_, '_, Separator, Context>
where
    Separator: Format<Context>,
{
    /// Adds a new node with the specified formatted content to the output, respecting any new lines
    /// that appear before the node in the input source.
    pub fn entry<L: Language>(&mut self, node: &SyntaxNode<L>, content: &dyn Format<Context>) {
        self.result = self.result.and_then(|_| {
            if self.has_elements {
                let n_lines = get_lines_before(node);

                match self.empty_lines {
                    EmptyLines::Single => {
                        if n_lines > 1 {
                            write!(self.fmt, [empty_line()])?;
                        } else {
                            self.separator.fmt(self.fmt)?;
                        }
                    }
                    // AIR: This branch is the main difference with the upstream variant.
                    // We use Biome's `empty_line()` to compress an arbitrary number of
                    // empty lines down to one, and then we force one more newline in.
                    // Note that if empty lines follow the element rather than
                    // precede it, this compression will not work as expected.
                    EmptyLines::Double =>
                    {
                        #[allow(clippy::comparison_chain)]
                        if n_lines > 2 {
                            write!(self.fmt, [empty_line(), text("\n")])?;
                        } else if n_lines == 2 {
                            write!(self.fmt, [empty_line()])?;
                        } else {
                            self.separator.fmt(self.fmt)?;
                        }
                    }
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
