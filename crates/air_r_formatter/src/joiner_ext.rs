use biome_formatter::{Format, FormatResult, prelude::*, write};
use biome_rowan::{Language, SyntaxNode};

/// The maximum number of empty lines allowed between elements
#[derive(Debug, Clone, Copy, Default)]
pub enum EmptyLines {
    #[default]
    Single,
    Double,
}

/// Version of `JoinNodesBuilder` that can be configured to respect up to two lines between inputs.
/// From https://github.com/biomejs/biome/blob/main/crates/biome_formatter/src/comments/builder.rs
#[must_use = "must eventually call `finish()` on Format builders"]
pub struct JoinNodesBuilderExt<'fmt, 'buf, Separator, Context> {
    result: FormatResult<()>,
    /// The separator to insert between nodes. Either a soft or hard line break
    separator: Separator,
    fmt: &'fmt mut Formatter<'buf, Context>,
    has_elements: bool,
    pending_lines_before: usize,
    empty_lines: EmptyLines,
}

impl<'fmt, 'buf, Separator, Context> JoinNodesBuilderExt<'fmt, 'buf, Separator, Context> {
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
            pending_lines_before: 0,
            empty_lines,
        }
    }
}

impl<Separator, Context> JoinNodesBuilderExt<'_, '_, Separator, Context>
where
    Separator: Format<Context>,
{
    /// Adds a new node with the specified formatted content to the output, respecting any new lines
    /// that appear before the node in the input source.
    pub fn entry<L: Language>(&mut self, node: &SyntaxNode<L>, content: &dyn Format<Context>) {
        self.result = self.result.and_then(|_| {
            if self.has_elements {
                let n_lines = self.pending_lines_before.max(get_lines_before(node));

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
            self.pending_lines_before = 0;

            write!(self.fmt, [content])
        });
    }

    /// Writes an entry without adding a separating line break or empty line
    ///
    /// Only used for `REmptyStatement`
    ///
    /// The entry doesn't count as an element, but the existing line breaks
    /// before it carry over to the next `entry()`. For example, with the
    /// following, the empty line before the `;` is kept due to this:
    ///
    /// ```r
    /// {
    ///   1
    ///
    ///   ;
    ///   2
    /// }
    /// ```
    ///
    /// We don't set `self.has_elements = true` so that in the following case,
    /// the `entry()` call to write `1` doesn't write a blank line at the start
    /// of the block (we want to remove all blank lines here):
    ///
    /// ```r
    /// {
    ///
    ///   ;
    ///   1
    /// }
    /// ```
    pub fn entry_no_separator<L: Language>(
        &mut self,
        node: &SyntaxNode<L>,
        content: &dyn Format<Context>,
    ) {
        self.result = self.result.and_then(|_| {
            self.pending_lines_before = self.pending_lines_before.max(get_lines_before(node));
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
