use crate::prelude::*;
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
            CommentTextPosition::EndOfLine => handle_for_comment(comment),
            CommentTextPosition::OwnLine => handle_for_comment(comment),
            CommentTextPosition::SameLine => handle_for_comment(comment),
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
