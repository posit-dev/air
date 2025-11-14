use air_r_syntax::AnyRExpression;
use air_r_syntax::RArgument;
use air_r_syntax::RArgumentNameClause;
use air_r_syntax::RElseClause;
use air_r_syntax::RForStatement;
use air_r_syntax::RFunctionDefinition;
use air_r_syntax::RIfStatement;
use air_r_syntax::RLanguage;
use air_r_syntax::RParenthesizedExpression;
use air_r_syntax::RRepeatStatement;
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
use biome_rowan::AstNode;
use biome_rowan::SyntaxTriviaPieceComments;

use crate::prelude::*;

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

    /// Must never be called!
    ///
    /// Would be called by [biome_formatter::Comments::is_suppressed()], i.e. by a call
    /// to `f.comments().is_suppressed()`. We can't use that, as it uses
    /// [biome_formatter::Comments::leading_dangling_trailing_comments()] and will check
    /// all three styles of comments for suppression. In Air, we only allow suppression
    /// on leading comments.
    ///
    /// If you are writing your own `is_suppressed()` method for a [FormatNodeRule], you
    /// instead want [crate::CommentDirectives::has_skip_directive()] to check if a node
    /// should be suppressed, combined with an explicit call to
    /// [biome_formatter::Comments::mark_suppression_checked()] to acknowledge that you've
    /// checked that node.
    fn is_suppression(_text: &str) -> bool {
        unreachable!("You want `CommentDirectives::has_skip_directive()` instead");
    }

    fn get_comment_kind(_comment: &SyntaxTriviaPieceComments<RLanguage>) -> CommentKind {
        // R doesn't have block or inline-block comments
        CommentKind::Line
    }

    fn place_comment(
        &self,
        comment: DecoratedComment<Self::Language>,
    ) -> CommentPlacement<Self::Language> {
        match comment.text_position() {
            CommentTextPosition::EndOfLine => handle_for_comment(comment)
                .or_else(handle_function_comment)
                .or_else(handle_while_comment)
                .or_else(handle_repeat_comment)
                .or_else(handle_if_statement_comment)
                .or_else(handle_else_clause_comment)
                .or_else(handle_parenthesized_expression_comment)
                .or_else(handle_argument_name_clause_comment)
                .or_else(handle_argument_comment)
                .or_else(handle_hole_argument_comment),
            CommentTextPosition::OwnLine => handle_for_comment(comment)
                .or_else(handle_function_comment)
                .or_else(handle_while_comment)
                .or_else(handle_repeat_comment)
                .or_else(handle_if_statement_comment)
                .or_else(handle_else_clause_comment)
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
    let Some(for_statement) = RForStatement::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };

    // If the following node is the `body`, we want to lead the first element of the body.
    // But we only want to do this if we are past the `)` of the loop sequence, so we have
    // to check that the following token isn't `)` too.
    //
    //
    // ```r
    // for (i in 1:5) # dangles {}
    //   {
    //   }
    // ```
    //
    // ```r
    // for (i in 1:5) # leads `a`
    // {
    //   a
    // }
    // ```
    //
    // Particularly important for prepping for autobracing here
    //
    // ```r
    // for (i in 1:5) # leads `a`
    //   a
    // ```
    //
    // This is consistent to when there are already braces there.
    //
    // ```r
    // for (i in 1:5) { # leads `a`
    //   a
    // }
    // ```
    //
    // Here the following node is the `body`, but the following token is `)`. We want
    // to avoid stealing this comment.
    //
    // ```r
    // for (i in 1:5 # comment
    // ) {
    // }
    // ```
    if let Ok(body) = for_statement.body() {
        if let Some(following) = comment.following_node() {
            if let Some(following_token) = comment.following_token() {
                if body.syntax() == following && following_token.kind() != RSyntaxKind::R_PAREN {
                    return place_leading_or_dangling_body_comment(body, comment);
                }
            }
        }
    }

    // Otherwise if the comment is "enclosed" by the `for_statement` then it can only be
    // in one of a few places, none of which are particularly meaningful, so we move the
    // comment to lead the whole `for_statement`
    //
    // ```r
    // for # comment
    // (i in 1:5) {
    // }
    // ```
    //
    // ```r
    // for ( # comment
    // i in 1:5) {
    // }
    // ```
    //
    // ```r
    // for (i # comment
    // in 1:5) {
    // }
    // ```
    //
    // ```r
    // for (i in # comment
    // 1:5) {
    // }
    // ```
    //
    // ```r
    // for (i in
    // # comment
    // 1:5) {
    // }
    // ```
    //
    // ```r
    // for (i in 1:5 # comment
    // ) {
    // }
    // ```
    CommentPlacement::leading(for_statement.into_syntax(), comment)
}

fn handle_while_comment(comment: DecoratedComment<RLanguage>) -> CommentPlacement<RLanguage> {
    let Some(while_statement) = RWhileStatement::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };

    // If the following node is the `body`, we want to lead the first element of the body.
    // But we only want to do this if we are past the `)` of the loop sequence, so we have
    // to check that the following token isn't `)` too.
    //
    //
    // ```r
    // while (condition) # dangles {}
    //   {
    //   }
    // ```
    //
    // ```r
    // while (condition) # leads `a`
    // {
    //   a
    // }
    // ```
    //
    // Particularly important for prepping for autobracing here
    //
    // ```r
    // while (condition) # leads `a`
    //   a
    // ```
    //
    // This is consistent to when there are already braces there.
    //
    // ```r
    // while (condition) { # leads `a`
    //   a
    // }
    // ```
    //
    // Here the following node is the `body`, but the following token is `)`. We want
    // to avoid stealing this comment.
    //
    // ```r
    // while (condition # comment
    // ) {
    // }
    // ```
    if let Ok(body) = while_statement.body() {
        if let Some(following) = comment.following_node() {
            if let Some(following_token) = comment.following_token() {
                if body.syntax() == following && following_token.kind() != RSyntaxKind::R_PAREN {
                    return place_leading_or_dangling_body_comment(body, comment);
                }
            }
        }
    }

    // Otherwise if the comment is "enclosed" by the `while_statement` then it can only be
    // in one of a few places, none of which are particularly meaningful, so we move the
    // comment to lead the whole `while_statement`
    //
    // ```r
    // while # comment
    // (condition) {
    // }
    // ```
    //
    // ```r
    // while ( # comment
    // condition) {
    // }
    // ```
    //
    // ```r
    // while (
    // # comment
    // condition) {
    // }
    // ```
    //
    // ```r
    // while (condition # comment
    // ) {
    // }
    // ```
    CommentPlacement::leading(while_statement.into_syntax(), comment)
}

fn handle_repeat_comment(comment: DecoratedComment<RLanguage>) -> CommentPlacement<RLanguage> {
    let Some(repeat_statement) = RRepeatStatement::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };

    // If the comment is "enclosed" by the `repeat_statement` then it can only be
    // in one of a few places, all of which should move the comment onto the first
    // element of the `body`
    //
    // ```r
    // repeat # dangles {}
    // {
    // }
    // ```
    //
    // ```r
    // repeat
    // # dangles {}
    // {
    // }
    // ```
    //
    // ```r
    // repeat # leads `a`
    // {
    //   a
    // }
    // ```
    //
    // Particularly important for prepping for autobracing here
    //
    // ```r
    // repeat # leads `a`
    //   a
    // ```
    //
    // This is consistent to when there are already braces there.
    //
    // ```r
    // repeat { # leads `a`
    //   a
    // }
    // ```
    if let Ok(body) = repeat_statement.body() {
        return place_leading_or_dangling_body_comment(body, comment);
    }

    // We don't expect to ever fall through to here, but if we do for some unknown reason
    // then we make the comment lead the whole `repeat_statement` to be consistent with
    // `for_statement` and `while_statement`
    CommentPlacement::leading(repeat_statement.into_syntax(), comment)
}

fn handle_function_comment(comment: DecoratedComment<RLanguage>) -> CommentPlacement<RLanguage> {
    let Some(function_definition) = RFunctionDefinition::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };

    // If the following node is the `parameters`, we make the comment lead the whole
    // `function_definition` node
    //
    // ```r
    // function # becomes leading on everything
    // () a
    // ```
    //
    // ```r
    // function
    // # becomes leading on everything
    // () a
    // ```
    if let Ok(parameters) = function_definition.parameters() {
        if let Some(following) = comment.following_node() {
            if parameters.syntax() == following {
                return CommentPlacement::leading(function_definition.into_syntax(), comment);
            }
        };
    };

    // If the following node is the `body`, we make the comment lead the first node of
    // the `body`.
    //
    // ```r
    // function() # becomes leading on `a`
    //   a
    // ```
    //
    // ```r
    // function() # becomes leading on `a`
    // {
    //   a
    // }
    // ```
    //
    // This is consistent with what happens when the comment is already after an opening
    // `{`
    //
    // ```r
    // function() { # becomes leading on `a`
    //   a
    // }
    // ```
    if let Ok(body) = function_definition.body() {
        if let Some(following) = comment.following_node() {
            if body.syntax() == following {
                return place_leading_or_dangling_body_comment(body, comment);
            }
        };
    };

    CommentPlacement::Default(comment)
}

fn handle_if_statement_comment(
    comment: DecoratedComment<RLanguage>,
) -> CommentPlacement<RLanguage> {
    let Some(if_statement) = RIfStatement::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };

    let Ok(condition) = if_statement.condition() else {
        // Bail with default placement if we don't have a `condition`, should
        // be unexpected
        return CommentPlacement::Default(comment);
    };

    let Ok(consequence) = if_statement.consequence() else {
        // Bail with default placement if we don't have a `consequence`, should
        // be unexpected
        return CommentPlacement::Default(comment);
    };

    // Make comments directly after the `condition` trailing
    // comments of the `condition` itself
    //
    // ```r
    // if (
    //   condition
    //   # comment
    // ) {
    // }
    // ```
    //
    // But don't steal comments that belong on the `consequence`, like
    // below. Here the preceding node is the `condition`. To do this right,
    // we also force the following token to be `)`.
    //
    // ```r
    // if (condition) # comment
    //   consequence
    // ```
    if let Some(preceding) = comment.preceding_node() {
        if let Some(following_token) = comment.following_token() {
            if condition.syntax() == preceding
                && matches!(following_token.kind(), RSyntaxKind::R_PAREN)
            {
                return CommentPlacement::trailing(preceding.clone(), comment);
            }
        }
    }

    // Figure out if this is a comment that comes directly before the
    // `consequence` and after the `)`, in which case we move it onto
    // the `consequence` to prepare for autobracing
    //
    // ```r
    // if (condition) # comment
    //   consequence
    // ```
    //
    // This aligns with behavior when braces already exist. Here, the comment's
    // default placement makes it leading on `consequence` (the enclosing node is
    // the braced expression, and there are no preceding nodes, so it attaches as leading
    // on the following node).
    //
    // ```r
    // if (condition) { # comment
    //   consequence
    // }
    // ```
    //
    // It is important that we first check for comments after the `condition`
    // but before the `)`, which we do above, otherwise this will steal this
    // comment and place it on `consequence` when it really belongs to `condition`
    //
    // ```r
    // if (
    //   condition
    //   # comment
    // ) {
    // }
    // ```
    if let Some(following) = comment.following_node() {
        if consequence.syntax() == following {
            return place_leading_or_dangling_body_comment(consequence, comment);
        }
    }

    // Move comments directly before an `else_clause` to the correct location
    //
    // Most `else` related handling is done by `handle_else_clause_comment()`,
    // but when the comment is right before the `else` token, it is "enclosed"
    // by the if statement node instead. We handle all of those cases here.
    //
    // We try extremely hard to force `} else` onto the same line with nothing
    // between them, for maximum portability of the if/else statement. This
    // requires moving the comment onto `consequence` or `alternative`.
    //
    // We greatly prefer creating leading comments over dangling comments when we move
    // them, as they play much nicer with idempotence.
    //
    // ```r
    // {
    //   if (cond) this # becomes leading on `this`
    //   else that
    // }
    // ```
    //
    // ```r
    // {
    //   if (cond) this
    //   # becomes leading on `that`
    //   else that
    // }
    // ```
    //
    // ```r
    // {
    //   if (cond) {
    //     this
    //   } # becomes leading on `this`
    //   else {
    //     that
    //   }
    // }
    // ```
    //
    // ```r
    // {
    //   if (cond) {
    //
    //   } # becomes dangling on {}
    //   else {
    //     that
    //   }
    // }
    // ```
    //
    // ```r
    // {
    //   if (cond) {
    //     this
    //   }
    //   # becomes leading on `that`
    //   else {
    //     that
    //   }
    // }
    // ```
    //
    // ```r
    // {
    //   if (cond) {
    //     this
    //   }
    //   # becomes dangling on `{}`
    //   else {
    //   }
    // }
    // ```
    //
    // ```r
    // {
    //   if (cond) {
    //     this
    //   }
    //   # becomes leading on `that`
    //   else if (cond) {
    //     that
    //   }
    // }
    // ```
    if let Some(else_clause) = if_statement.else_clause() {
        let Ok(alternative) = else_clause.alternative() else {
            // Bail with default placement if we don't have a `alternative`, should
            // be unexpected
            return CommentPlacement::Default(comment);
        };

        if let Some(following) = comment.following_node() {
            if else_clause.syntax() == following {
                return match comment.text_position() {
                    // End of line comments lead the `consequence` body
                    CommentTextPosition::EndOfLine => {
                        place_leading_or_dangling_body_comment(consequence, comment)
                    }
                    // Own line comments lead the `alternative` body
                    CommentTextPosition::OwnLine => {
                        place_leading_or_dangling_alternative_comment(alternative, comment)
                    }
                    CommentTextPosition::SameLine => {
                        unreachable!("Inline comments aren't possible in R")
                    }
                };
            }
        }
    }

    CommentPlacement::Default(comment)
}

fn handle_else_clause_comment(comment: DecoratedComment<RLanguage>) -> CommentPlacement<RLanguage> {
    let Some(else_clause) = RElseClause::cast_ref(comment.enclosing_node()) else {
        return CommentPlacement::Default(comment);
    };

    let Ok(alternative) = else_clause.alternative() else {
        // Bail with default placement if we don't have a `alternative`, should
        // be unexpected
        return CommentPlacement::Default(comment);
    };

    // Comments following the `else` token but before the `alternative` are enclosed by
    // the `else_clause`. We make these comments leading on the `alternative`.
    //
    // ```r
    // {
    //   if (condition) a
    //   else # becomes leading on `b`
    //   {
    //     b
    //   }
    // }
    // ```
    //
    // ```r
    // {
    //   if (condition) a
    //   else
    //   # becomes leading on `b`
    //   {
    //     b
    //   }
    // }
    // ```
    if let Some(following) = comment.following_node() {
        if alternative.syntax() == following {
            return place_leading_or_dangling_alternative_comment(alternative, comment);
        }
    }

    CommentPlacement::Default(comment)
}

/// Basically [place_leading_or_dangling_body_comment()], but moves comments on an if
/// statement `alternative` onto the body of that if statement to handle if chaining a bit
/// nicer
///
/// ```r
/// {
///   if (condition) {
///     a
///   } else # becomes leading on `b` rather than leading on if statement `alternative`
///   if (condition) {
///     b
///   }
/// }
/// ```
fn place_leading_or_dangling_alternative_comment(
    alternative: AnyRExpression,
    comment: DecoratedComment<RLanguage>,
) -> CommentPlacement<RLanguage> {
    match alternative {
        AnyRExpression::RIfStatement(node) => match node.consequence() {
            Ok(consequence) => place_leading_or_dangling_body_comment(consequence, comment),
            Err(_) => CommentPlacement::Default(comment),
        },
        node => place_leading_or_dangling_body_comment(node, comment),
    }
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
            if let Some(following) = comment.following_node() {
                return CommentPlacement::leading(following.clone(), comment);
            } else {
                return CommentPlacement::leading(preceding.clone(), comment);
            }
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
/// if (cond) # becomes dangling on `{}`
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
