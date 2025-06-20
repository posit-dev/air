use air_r_syntax::RLanguage;
use air_r_syntax::RSyntaxNode;
use air_r_syntax::RSyntaxToken;
use biome_formatter::CstFormatContext;
use biome_formatter::FormatLanguage;
use biome_formatter::FormatOwnedWithRule;
use biome_formatter::FormatRefWithRule;
use biome_formatter::FormatToken;
use biome_formatter::Formatted;
use biome_formatter::TransformSourceMap;
use biome_formatter::comments::Comments;
use biome_formatter::prelude::*;
use biome_formatter::write;
use biome_rowan::AstNode;

use crate::comments::RCommentStyle;
use crate::context::RFormatContext;
use crate::context::RFormatOptions;
use crate::cst::FormatRSyntaxNode;

pub mod comments;
pub mod context;
mod cst;
pub mod either;
pub mod formatter_ext;
pub mod joiner_ext;
pub mod loop_body;
mod prelude;
mod r;
pub(crate) mod separated;
mod string_literal;

#[rustfmt::skip]
mod generated;

/// Used to get an object that knows how to format this object.
pub(crate) trait AsFormat<Context> {
    type Format<'a>: biome_formatter::Format<Context>
    where
        Self: 'a;

    /// Returns an object that is able to format this object.
    fn format(&self) -> Self::Format<'_>;
}

/// Implement [AsFormat] for references to types that implement [AsFormat].
impl<T, C> AsFormat<C> for &T
where
    T: AsFormat<C>,
{
    type Format<'a>
        = T::Format<'a>
    where
        Self: 'a;

    fn format(&self) -> Self::Format<'_> {
        AsFormat::format(&**self)
    }
}

/// Implement [AsFormat] for [SyntaxResult] where `T` implements [AsFormat].
///
/// Useful to format mandatory AST fields without having to unwrap the value first.
impl<T, C> AsFormat<C> for biome_rowan::SyntaxResult<T>
where
    T: AsFormat<C>,
{
    type Format<'a>
        = biome_rowan::SyntaxResult<T::Format<'a>>
    where
        Self: 'a;

    fn format(&self) -> Self::Format<'_> {
        match self {
            Ok(value) => Ok(value.format()),
            Err(err) => Err(*err),
        }
    }
}

/// Implement [AsFormat] for [Option] when `T` implements [AsFormat]
///
/// Allows to call format on optional AST fields without having to unwrap the field first.
impl<T, C> AsFormat<C> for Option<T>
where
    T: AsFormat<C>,
{
    type Format<'a>
        = Option<T::Format<'a>>
    where
        Self: 'a;

    fn format(&self) -> Self::Format<'_> {
        self.as_ref().map(|value| value.format())
    }
}

/// Used to convert this object into an object that can be formatted.
///
/// The difference to [AsFormat] is that this trait takes ownership of `self`.
// False positive
#[allow(dead_code)]
pub(crate) trait IntoFormat<Context> {
    type Format: biome_formatter::Format<Context>;

    fn into_format(self) -> Self::Format;
}

impl<T, Context> IntoFormat<Context> for biome_rowan::SyntaxResult<T>
where
    T: IntoFormat<Context>,
{
    type Format = biome_rowan::SyntaxResult<T::Format>;

    fn into_format(self) -> Self::Format {
        self.map(IntoFormat::into_format)
    }
}

/// Implement [IntoFormat] for [Option] when `T` implements [IntoFormat]
///
/// Allows to call format on optional AST fields without having to unwrap the field first.
impl<T, Context> IntoFormat<Context> for Option<T>
where
    T: IntoFormat<Context>,
{
    type Format = Option<T::Format>;

    fn into_format(self) -> Self::Format {
        self.map(IntoFormat::into_format)
    }
}

/// Formatting specific [Iterator] extensions
// False positive
#[allow(dead_code)]
pub(crate) trait FormattedIterExt {
    /// Converts every item to an object that knows how to format it.
    fn formatted<Context>(self) -> FormattedIter<Self, Self::Item, Context>
    where
        Self: Iterator + Sized,
        Self::Item: IntoFormat<Context>,
    {
        FormattedIter {
            inner: self,
            options: std::marker::PhantomData,
        }
    }
}

impl<I> FormattedIterExt for I where I: std::iter::Iterator {}

// False positive
#[allow(dead_code)]
pub(crate) struct FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item>,
{
    inner: Iter,
    options: std::marker::PhantomData<Context>,
}

impl<Iter, Item, Context> std::iter::Iterator for FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item>,
    Item: IntoFormat<Context>,
{
    type Item = Item::Format;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.inner.next()?.into_format())
    }
}

impl<Iter, Item, Context> std::iter::FusedIterator for FormattedIter<Iter, Item, Context>
where
    Iter: std::iter::FusedIterator<Item = Item>,
    Item: IntoFormat<Context>,
{
}

impl<Iter, Item, Context> std::iter::ExactSizeIterator for FormattedIter<Iter, Item, Context>
where
    Iter: Iterator<Item = Item> + std::iter::ExactSizeIterator,
    Item: IntoFormat<Context>,
{
}

pub(crate) type RFormatter<'buf> = Formatter<'buf, RFormatContext>;

/// Rule for formatting an R [AstNode].
pub(crate) trait FormatNodeRule<N>
where
    N: AstNode<Language = RLanguage>,
{
    // This is the method that actually starts the formatting
    fn fmt(&self, node: &N, f: &mut RFormatter) -> FormatResult<()> {
        if self.is_suppressed(node, f) {
            return self.fmt_suppressed(node, f);
        }

        self.fmt_leading_comments(node, f)?;
        self.fmt_fields(node, f)?;
        self.fmt_dangling_comments(node, f)?;
        self.fmt_trailing_comments(node, f)
    }

    fn fmt_fields(&self, node: &N, f: &mut RFormatter) -> FormatResult<()>;

    /// Returns `true` if the node is suppressed and should use the same formatting as in the source document.
    fn is_suppressed(&self, node: &N, f: &RFormatter) -> bool {
        is_suppressed_by_comment(node, f)
    }

    /// Formats a suppressed node
    ///
    /// You may want to override this method if you need to manually handle
    /// formatting of a suppressed node. This should be extremely rare.
    fn fmt_suppressed(&self, node: &N, f: &mut RFormatter) -> FormatResult<()> {
        write!(f, [format_suppressed_node(node.syntax())])
    }

    /// Formats the [leading comments](biome_formatter::comments#leading-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the leading comments.
    fn fmt_leading_comments(&self, node: &N, f: &mut RFormatter) -> FormatResult<()> {
        format_leading_comments(node.syntax()).fmt(f)
    }

    /// Formats the [dangling comments](biome_formatter::comments#dangling-comments) of the node.
    ///
    /// You should override this method if the node handled by this rule can have dangling comments because the
    /// default implementation formats the dangling comments at the end of the node, which isn't ideal but ensures that
    /// no comments are dropped.
    ///
    /// A node can have dangling comments if all its children are tokens or if all node childrens are optional.
    fn fmt_dangling_comments(&self, node: &N, f: &mut RFormatter) -> FormatResult<()> {
        format_dangling_comments(node.syntax())
            .with_soft_block_indent()
            .fmt(f)
    }

    /// Formats the [trailing comments](biome_formatter::comments#trailing-comments) of the node.
    ///
    /// You may want to override this method if you want to manually handle the formatting of comments
    /// inside of the `fmt_fields` method or customize the formatting of the trailing comments.
    fn fmt_trailing_comments(&self, node: &N, f: &mut RFormatter) -> FormatResult<()> {
        format_trailing_comments(node.syntax()).fmt(f)
    }
}

/// Returns `true` if the node has a suppression comment and should use the same formatting as in the source document.
///
/// Calls [biome_formatter::comments::Comments::mark_suppression_checked] on `node`.
#[inline]
pub(crate) fn is_suppressed_by_comment<N>(node: &N, f: &RFormatter) -> bool
where
    N: AstNode<Language = RLanguage>,
{
    f.context().comments().is_suppressed(node.syntax())
}

/// Rule for formatting an bogus node.
pub(crate) trait FormatBogusNodeRule<N>
where
    N: AstNode<Language = RLanguage>,
{
    fn fmt(&self, node: &N, f: &mut RFormatter) -> FormatResult<()> {
        format_bogus_node(node.syntax()).fmt(f)
    }
}

/// Format implementation specific to R tokens.
pub(crate) type FormatRSyntaxToken = FormatToken<RFormatContext>;

impl AsFormat<RFormatContext> for RSyntaxToken {
    type Format<'a> = FormatRefWithRule<'a, RSyntaxToken, FormatRSyntaxToken>;

    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, FormatRSyntaxToken::default())
    }
}

impl IntoFormat<RFormatContext> for RSyntaxToken {
    type Format = FormatOwnedWithRule<RSyntaxToken, FormatRSyntaxToken>;

    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, FormatRSyntaxToken::default())
    }
}

#[derive(Debug, Clone)]
pub struct RFormatLanguage {
    options: RFormatOptions,
}
impl RFormatLanguage {
    pub fn new(options: RFormatOptions) -> Self {
        Self { options }
    }
}

impl FormatLanguage for RFormatLanguage {
    type SyntaxLanguage = RLanguage;
    type Context = RFormatContext;
    type FormatRule = FormatRSyntaxNode;

    fn options(&self) -> &RFormatOptions {
        &self.options
    }

    fn create_context(
        self,
        root: &RSyntaxNode,
        source_map: Option<TransformSourceMap>,
    ) -> Self::Context {
        let comments = Comments::from_node(root, &RCommentStyle, source_map.as_ref());
        RFormatContext::new(self.options, comments).with_source_map(source_map)
    }
}

/// Formats an R syntax tree.
///
/// It returns the [Formatted] result that can be printed to a string.
pub fn format_node(
    options: RFormatOptions,
    root: &RSyntaxNode,
) -> FormatResult<Formatted<RFormatContext>> {
    biome_formatter::format_node(root, RFormatLanguage::new(options))
}
