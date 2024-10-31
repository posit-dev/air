use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RLanguage;
use air_r_syntax::RSyntaxKind;
use biome_formatter::comments::CommentKind;
use biome_formatter::comments::CommentPlacement;
use biome_formatter::comments::CommentStyle;
use biome_formatter::comments::CommentTextPosition;
use biome_formatter::comments::Comments;
use biome_formatter::comments::DecoratedComment;
use biome_formatter::comments::SourceComment;
use biome_formatter::write;
use biome_rowan::SyntaxTriviaPieceComments;

pub type RComments = Comments<RLanguage>;

#[derive(Default)]
pub struct FormatRLeadingComment;

impl FormatRule<SourceComment<RLanguage>> for FormatRLeadingComment {
    type Context = RFormatContext;

    fn fmt(
        &self,
        comment: &SourceComment<RLanguage>,
        f: &mut Formatter<Self::Context>,
    ) -> FormatResult<()> {
        // It seems like this is only worth customizing for multi-line comments
        // that need to be aligned together
        write!(f, [comment.piece().as_piece()])
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug, Default)]
pub struct RCommentStyle;

impl CommentStyle for RCommentStyle {
    type Language = RLanguage;

    fn is_suppression(_text: &str) -> bool {
        // TODO: Implement ark format suppression
        false
    }

    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<RLanguage>) -> CommentKind {
        // R doesn't have block or inline-block comments
        CommentKind::Line
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        // TODO: Implement more rule based comment placement, see `biome_js_formatter`
        match comment.text_position() {
            CommentTextPosition::EndOfLine => {
                handle_for_comment(comment).or_else(handle_function_comment)
            }
            CommentTextPosition::OwnLine => {
                handle_for_comment(comment).or_else(handle_function_comment)
            }
            CommentTextPosition::SameLine => {
                // Not applicable for R, we don't have `/* */` comments
                CommentPlacement::Default(comment)
            }
        }
    }
}

fn handle_for_comment(comment: DecoratedComment<RLanguage>) -> CommentPlacement<RLanguage> {
    let enclosing = comment.enclosing_node();

    if enclosing.kind() != RSyntaxKind::R_FOR_STATEMENT {
        return CommentPlacement::Default(comment);
    }

    if comment.text_position().is_own_line() {
        // Lift comment up as a leading comment on the whole `R_FOR_STATEMENT` node
        return CommentPlacement::leading(enclosing.clone(), comment);
    }

    CommentPlacement::Default(comment)
}

fn handle_function_comment(comment: DecoratedComment<RLanguage>) -> CommentPlacement<RLanguage> {
    if !matches!(
        comment.enclosing_node().kind(),
        RSyntaxKind::R_FUNCTION_DEFINITION
    ) {
        return CommentPlacement::Default(comment);
    };

    // Function definitions have `name`, `parameters`, and `body` fields, and
    // only the `body` can be an `AnyRExpression`.
    let Some(body) = comment.following_node().and_then(AnyRExpression::cast_ref) else {
        return CommentPlacement::Default(comment);
    };

    // Make line comments between the `)` token and the function body:
    // - Leading comments of the first expression within `{}`
    // - Dangling comments of the `{}` if they are empty
    // - Leading comments of the body if the body is not a `{}` expression
    //
    // Doing this allows these comments to be handled elegantly in one pass.
    // Otherwise we can end up with unstable formatting where in a first pass we
    // format as:
    //
    // ```r
    // function() { # comment
    // }
    // ```
    //
    // and then in a second pass we format as:
    //
    // ```r
    // function() {
    //   # comment
    // }
    // ```
    //
    // Examples:
    //
    // ```r
    // function() # becomes leading on `1 + 1`
    // {
    //  1 + 1
    // }
    // ```
    //
    // ```r
    // function() # becomes dangling on the `{}`
    // {
    // }
    // ```
    //
    // ```r
    // function() # becomes leading on `1 + 1`
    //   1 + 1
    // ```
    match body {
        AnyRExpression::RBracedExpressions(body) => match body.expressions().first() {
            Some(first) => CommentPlacement::leading(first.into_syntax(), comment),
            None => CommentPlacement::dangling(body.into_syntax(), comment),
        },
        _ => CommentPlacement::leading(body.into_syntax(), comment),
    }
}
