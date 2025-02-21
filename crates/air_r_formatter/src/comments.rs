use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RArgument;
use air_r_syntax::RArgumentNameClause;
use air_r_syntax::RIfStatement;
use air_r_syntax::RLanguage;
use air_r_syntax::RParenthesizedExpression;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::RSyntaxToken;
use air_r_syntax::RWhileStatement;
use biome_formatter::comments::CommentKind;
use biome_formatter::comments::CommentPlacement;
use biome_formatter::comments::CommentStyle;
use biome_formatter::comments::CommentTextPosition;
use biome_formatter::comments::Comments;
use biome_formatter::comments::DecoratedComment;
use biome_formatter::comments::SourceComment;
use biome_formatter::write;
use biome_rowan::SyntaxTriviaPieceComments;
use comments::Directive;
use comments::FormatDirective;

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

    fn is_suppression(text: &str) -> bool {
        comments::parse_comment_directive(text)
            .is_some_and(|directive| matches!(directive, Directive::Format(FormatDirective::Skip)))
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
            CommentTextPosition::EndOfLine => handle_for_comment(comment)
                .or_else(handle_function_comment)
                .or_else(handle_while_comment)
                .or_else(handle_repeat_comment)
                .or_else(handle_if_statement_comment)
                .or_else(handle_parenthesized_expression_comment)
                .or_else(handle_argument_name_clause_comment)
                .or_else(handle_argument_comment)
                .or_else(handle_hole_argument_comment),
            CommentTextPosition::OwnLine => handle_for_comment(comment)
                .or_else(handle_function_comment)
                .or_else(handle_while_comment)
                .or_else(handle_repeat_comment)
                .or_else(handle_if_statement_comment)
                .or_else(handle_parenthesized_expression_comment)
                .or_else(handle_argument_name_clause_comment)
                .or_else(handle_argument_comment)
                .or_else(handle_hole_argument_comment),
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

fn handle_while_comment(comment: DecoratedComment<RLanguage>) -> CommentPlacement<RLanguage> {
    let Some(enclosing) = RWhileStatement::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };

    if let Some(preceding) = comment.preceding_node() {
        // Make comments directly before the condition `)` trailing
        // comments of the condition itself (rather than leading comments of
        // the `body` node)
        //
        // ```r
        // while (
        //   cond
        //   # comment
        // ) {
        // }
        // ```
        if comment
            .following_token()
            .map_or(false, |token| token.kind() == RSyntaxKind::R_PAREN)
        {
            return CommentPlacement::trailing(preceding.clone(), comment);
        }
    }

    // Check that the `body` of the while loop is identical to the `following`
    // node of the comment. While loops also have a `condition` that can be
    // any R expression, so we need to differentiate here.
    let Ok(body) = enclosing.body() else {
        return CommentPlacement::Default(comment);
    };
    let Some(following) = comment.following_node() else {
        return CommentPlacement::Default(comment);
    };
    if body.syntax() != following {
        return CommentPlacement::Default(comment);
    }

    // Handle cases like:
    //
    // ```r
    // while (a) # comment
    // {}
    // ```
    place_leading_or_dangling_body_comment(body, comment)
}

fn handle_repeat_comment(comment: DecoratedComment<RLanguage>) -> CommentPlacement<RLanguage> {
    if !matches!(
        comment.enclosing_node().kind(),
        RSyntaxKind::R_REPEAT_STATEMENT
    ) {
        return CommentPlacement::Default(comment);
    };

    // Repeat statements have a `repeat` token and a `body` field, and
    // only the `body` can be an `AnyRExpression`.
    let Some(body) = comment.following_node().and_then(AnyRExpression::cast_ref) else {
        return CommentPlacement::Default(comment);
    };

    // Handle cases like:
    //
    // ```r
    // repeat # comment
    // {}
    // ```
    place_leading_or_dangling_body_comment(body, comment)
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

    place_leading_or_dangling_body_comment(body, comment)
}

fn handle_if_statement_comment(
    comment: DecoratedComment<RLanguage>,
) -> CommentPlacement<RLanguage> {
    match (comment.enclosing_node().kind(), comment.following_node()) {
        (RSyntaxKind::R_IF_STATEMENT, Some(following)) => {
            let if_statement = RIfStatement::unwrap_cast(comment.enclosing_node().clone());

            if let Some(preceding) = comment.preceding_node() {
                // Make comments directly before the condition `)` trailing
                // comments of the condition itself
                //
                // ```r
                // if (
                //   cond
                //   # comment
                // ) {
                // }
                // ```
                if comment
                    .following_token()
                    .map_or(false, |token| token.kind() == RSyntaxKind::R_PAREN)
                {
                    return CommentPlacement::trailing(preceding.clone(), comment);
                }
            }

            // Figure out if this is a comment that comes directly before the
            // `consequence` and after the `)`, in which case we move it onto
            // the `consequence`
            //
            // ```r
            // if (cond) # comment
            //   TRUE
            // ```
            if let Ok(consequence) = if_statement.consequence() {
                if consequence.syntax() == following {
                    return place_leading_or_dangling_body_comment(consequence, comment);
                }
            }
        }
        (RSyntaxKind::R_ELSE_CLAUSE, _) => {
            // TODO: Handle else clause comments in some way? See JS for an example.
            // fall through
        }
        _ => {
            // fall through
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_parenthesized_expression_comment(
    comment: DecoratedComment<RLanguage>,
) -> CommentPlacement<RLanguage> {
    let Some(enclosing) = RParenthesizedExpression::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };

    let Ok(body) = enclosing.body() else {
        // Should always have a `body`
        return CommentPlacement::Default(comment);
    };

    if let Some(following) = comment.following_node() {
        // Make comments directly before the `body` leading comments of the `body`
        //
        // ```r
        // ( # comment
        //   body
        // )
        // ```
        if body.syntax() == following {
            return CommentPlacement::leading(following.clone(), comment);
        }
    }

    if let Some(preceding) = comment.preceding_node() {
        // Make comments directly after the `body` trailing comments of the `body`
        //
        // ```r
        // (
        //   body
        //   # comment
        // )
        // ```
        if body.syntax() == preceding {
            return CommentPlacement::trailing(preceding.clone(), comment);
        }
    }

    // Likely not possible
    CommentPlacement::Default(comment)
}

fn handle_argument_name_clause_comment(
    comment: DecoratedComment<RLanguage>,
) -> CommentPlacement<RLanguage> {
    let Some(enclosing) = RArgumentNameClause::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };

    let Ok(name) = enclosing.name() else {
        // We expect to always have a `name` and never fall through here
        return CommentPlacement::Default(comment);
    };

    if let Some(preceding) = comment.preceding_node() {
        // Make comments directly after the `name` leading comments of the `name`
        //
        // Needed for idempotence.
        //
        // ```r
        // fn(
        //  xs,
        //  a # end-of-line
        //    = expr
        // )
        // ```
        if name.syntax() == preceding {
            return CommentPlacement::leading(preceding.clone(), comment);
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_argument_comment(comment: DecoratedComment<RLanguage>) -> CommentPlacement<RLanguage> {
    let Some(enclosing) = RArgument::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };

    let Some(name_clause) = enclosing.name_clause() else {
        // Don't need to worry about comment placement on unnamed arguments
        return CommentPlacement::Default(comment);
    };

    if let Some(preceding) = comment.preceding_node() {
        // Make comments directly after the `name_clause` leading comments of
        // the `name_clause`
        //
        // Needed for idempotence.
        //
        // ```r
        // fn(
        //  xs,
        //  a = # end-of-line
        //    expr
        // )
        // ```
        //
        // ```r
        // fn(
        //  xs,
        //  a =
        //    # own-line
        //    expr
        // )
        // ```
        if name_clause.syntax() == preceding {
            return CommentPlacement::leading(preceding.clone(), comment);
        }
    }

    CommentPlacement::Default(comment)
}

/// Handle comments close to an argument hole
///
/// Hole comment handling is complicated by the fact that holes don't have any
/// associated tokens. This has two important consequences that we exploit:
///
/// - Comments are never enclosed by holes, i.e. `enclosing_node()` never
///   returns a hole.
///
/// - Comments will ALWAYS trail the hole by default, i.e. `preceding_node()`
///   is how you access the hole node connected to the comment.
///
/// Here is the logic for placing hole comments, assuming that we've already
/// determined that the `preceding_node()` is a hole. Note that this logic is
/// symmetric, which we quite like:
///
/// - If the following token is `,`, `)`, `]`, or `]]`, the comment is
///   "inside" the hole.
///
///   - If the previous sibling node before the hole exists and is not another
///     hole, attach the comment as trailing trivia on that.
///
/// - Else the comment is "after" the hole.
///
///   - If the following sibling node after the hole exists and is not another
///     hole, attach the comment as leading trivia on that.
///
/// - Otherwise attach the comment as leading on the hole itself. This happens
///   when there is not another preceding/following node, or that node is
///   another hole.
///
/// Note that the "following token" check skips following whitespace and
/// comments, which is what we want.
///
/// ## Comments "inside" the hole
///
/// Comment trails `a`. Following token is `,` and previous-sibling of hole is a
/// non-hole.
///
/// ```r
/// fn(
///   a,<hole> # comment
///   ,
///   b
/// )
/// ```
///
/// CommentB trails `b`. Following token is `)` and previous-sibling of hole is
/// a non-hole.
///
/// This example is particularly important. We want comments on trailing
/// `,` in `list2()` calls to get attached to the `b` non-hole node,
/// otherwise it will get moved to the next line if it stays attached to
/// the hole.
///
/// ```r
/// list2(
///   a, # commentA
///   b,<hole> # commentB
/// )
/// ```
///
/// Comment1 leads hole. Following token is `,` and there is no previous-sibling
/// of the hole. Note that `following_token()` skips `# comment2` here and jumps
/// straight to `,`, which is what we want.
///
/// Comment2 leads hole. Following token is `,` and there is no previous-sibling
/// of the hole.
///
/// ```r
/// fn(<hole># comment1
///  # comment2
///  ,
///  x
/// )
/// ```
///
/// Comment leads hole. Following token is `,` and the previous-sibling of
/// the hole is another hole.
///
/// ```r
/// fn(
///  a,<another-hole>
///  ,<hole># comment
///  ,
///  b
/// )
/// ```
///
/// Comment leads hole. Following token is `,` and the previous-sibling of
/// the hole is another hole.
///
/// ```r
/// fn(<another-hole>
///  ,<hole># comment
///  ,
///  x
/// )
/// ```
///
/// ## Comment "after" the hole
///
/// Comment leads `x`. Following token is not `,`, `)`, `]`, or `]]`, and the
/// following-sibling of the hole is a non-hole we can lead.
///
/// ```r
/// fn(
///  ,<hole>
///  ,# comment
///  x
/// )
/// ```
///
/// Comment1 leads `x`. Following token is not `,`, `)`, `]`, or `]]`, and the
/// following-sibling of the hole is a non-hole we can lead. Note that
/// `following_token()` skips `# comment2` here and jumps straight to `,`, which
/// is what we want.
///
/// Comment2 leads `x`. Following token is not `,`, `)`, `]`, or `]]`, and the
/// following-sibling of the hole is a non-hole we can lead.
///
/// ```r
/// fn(
///   ,<hole>
///   ,# comment1
///   # comment2
///   x
/// )
/// ```
///
/// We can't think of any scenario where we have a comment "after" the hole,
/// but we don't have a following-sibling to lead.
fn handle_hole_argument_comment(
    comment: DecoratedComment<RLanguage>,
) -> CommentPlacement<RLanguage> {
    let Some(hole) = comment
        .preceding_node()
        .and_then(RArgument::cast_ref)
        .filter(RArgument::is_hole)
        .map(RArgument::into_syntax)
    else {
        return CommentPlacement::Default(comment);
    };

    // Note that `following_token()` nicely skips over following comments
    let is_comment_inside_hole = matches!(
        comment.following_token().map(RSyntaxToken::kind),
        Some(
            RSyntaxKind::COMMA
                | RSyntaxKind::R_PAREN
                | RSyntaxKind::R_BRACK
                | RSyntaxKind::R_BRACK2
        )
    );

    #[allow(clippy::collapsible_else_if)]
    if is_comment_inside_hole {
        if let Some(previous) = hole
            .prev_sibling()
            .and_then(RArgument::cast)
            .filter(|argument| !argument.is_hole())
            .map(RArgument::into_syntax)
        {
            return CommentPlacement::trailing(previous, comment);
        }
    } else {
        if let Some(following) = comment
            .following_node()
            .and_then(RArgument::cast_ref)
            .filter(|argument| !argument.is_hole())
            .map(RArgument::into_syntax)
        {
            return CommentPlacement::leading(following, comment);
        }
    }

    CommentPlacement::leading(hole, comment)
}

/// Make line comments between a `)` token and a `body`:
/// - Leading comments of the first expression within `{}` if `body` is a braced expression
/// - Dangling comments of the `{}` if `body` is an empty braced expression
/// - Leading comments of the `body` if the `body` is not a braced expression
///
/// Doing this allows these comments to be handled elegantly in one pass.
/// Otherwise we can end up with unstable formatting where in a first pass we
/// format as:
///
/// ```r
/// function() { # comment
/// }
/// ```
///
/// and then in a second pass we format as:
///
/// ```r
/// function() {
///   # comment
/// }
/// ```
///
/// Examples:
///
/// ```r
/// function() # becomes leading on `1 + 1`
/// {
///  1 + 1
/// }
/// ```
///
/// ```r
/// function() # becomes dangling on the `{}`
/// {
/// }
/// ```
///
/// ```r
/// function() # becomes leading on `1 + 1`
///   1 + 1
/// ```
///
/// ```r
/// if (cond) # becomes leading on `{}`
/// {
/// }
/// ```
fn place_leading_or_dangling_body_comment(
    body: AnyRExpression,
    comment: DecoratedComment<RLanguage>,
) -> CommentPlacement<RLanguage> {
    match body {
        AnyRExpression::RBracedExpressions(body) => match body.expressions().first() {
            Some(first) => CommentPlacement::leading(first.into_syntax(), comment),
            None => CommentPlacement::dangling(body.into_syntax(), comment),
        },
        _ => CommentPlacement::leading(body.into_syntax(), comment),
    }
}
