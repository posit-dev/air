use crate::prelude::*;
use biome_formatter::comments::CommentKind;
use biome_formatter::comments::CommentPlacement;
use biome_formatter::comments::CommentStyle;
use biome_formatter::comments::Comments;
use biome_formatter::comments::DecoratedComment;
use biome_formatter::comments::SourceComment;
use biome_formatter::write;
use biome_r_syntax::RLanguage;
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
        // TODO: Implement rule based comment placement, see `biome_js_formatter`
        CommentPlacement::Default(comment)
    }
}
