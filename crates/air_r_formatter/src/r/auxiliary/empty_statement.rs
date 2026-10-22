use crate::prelude::*;
use air_r_syntax::REmptyStatement;
use air_r_syntax::REmptyStatementFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatREmptyStatement;
impl FormatNodeRule<REmptyStatement> for FormatREmptyStatement {
    fn fmt_fields(&self, node: &REmptyStatement, f: &mut RFormatter) -> FormatResult<()> {
        let REmptyStatementFields { semicolon_token } = node.as_fields();

        // Comment placement preemptively moves comments off of empty statements
        debug_assert!(
            !f.comments().has_comments(node.syntax()),
            "`REmptyStatement` can't have comments."
        );

        // We never write semicolons
        write!(f, [format_removed(&semicolon_token?)])
    }
}

// TODO(aether): Replace with `statements/semicolons.R` spec tests on the swap to aether
#[cfg(test)]
mod test {
    use air_r_factory::r_argument;
    use air_r_factory::r_argument_list;
    use air_r_factory::r_braced_statements;
    use air_r_factory::r_call;
    use air_r_factory::r_call_arguments;
    use air_r_factory::r_double_value;
    use air_r_factory::r_empty_statement;
    use air_r_factory::r_expression_statement;
    use air_r_factory::r_function_definition;
    use air_r_factory::r_identifier;
    use air_r_factory::r_if_statement;
    use air_r_factory::r_parameter_list;
    use air_r_factory::r_parameters;
    use air_r_factory::r_root;
    use air_r_factory::r_statement_list;
    use air_r_parser::RParserOptions;
    use air_r_syntax::AnyRExpression;
    use air_r_syntax::AnyRStatement;
    use air_r_syntax::RRoot;
    use air_r_syntax::RSyntaxKind;
    use air_r_syntax::RSyntaxKind::*;
    use air_r_syntax::RSyntaxToken;
    use biome_rowan::AstNode;
    use biome_rowan::TextSize;
    use biome_rowan::TriviaPiece;
    use biome_rowan::TriviaPieceKind;

    use crate::context::RFormatOptions;
    use crate::format_node;

    #[derive(Clone, Copy)]
    enum Trivia {
        Newline,
        Whitespace(&'static str),
        Comment(&'static str),
    }

    use Trivia::Comment;
    use Trivia::Newline;

    const INDENT: Trivia = Trivia::Whitespace("  ");
    const SPACE: Trivia = Trivia::Whitespace(" ");

    impl Trivia {
        fn text(self) -> &'static str {
            match self {
                Trivia::Newline => "\n",
                Trivia::Whitespace(text) | Trivia::Comment(text) => text,
            }
        }

        fn piece(self) -> TriviaPiece {
            let kind = match self {
                Trivia::Newline => TriviaPieceKind::Newline,
                Trivia::Whitespace(_) => TriviaPieceKind::Whitespace,
                Trivia::Comment(_) => TriviaPieceKind::SingleLineComment,
            };
            TriviaPiece::new(kind, TextSize::of(self.text()))
        }
    }

    fn token(
        kind: RSyntaxKind,
        leading: &[Trivia],
        text: &str,
        trailing: &[Trivia],
    ) -> RSyntaxToken {
        let text: String = leading
            .iter()
            .map(|trivia| trivia.text())
            .chain([text])
            .chain(trailing.iter().map(|trivia| trivia.text()))
            .collect();

        RSyntaxToken::new_detached(
            kind,
            &text,
            leading.iter().map(|trivia| trivia.piece()),
            trailing.iter().map(|trivia| trivia.piece()),
        )
    }

    fn root(
        statements: impl IntoIterator<Item = AnyRStatement, IntoIter: ExactSizeIterator>,
    ) -> RRoot {
        r_root(r_statement_list(statements), token(EOF, &[], "", &[])).build()
    }

    fn braces(
        l_curly_token: RSyntaxToken,
        statements: impl IntoIterator<Item = AnyRStatement, IntoIter: ExactSizeIterator>,
        r_curly_token: RSyntaxToken,
    ) -> AnyRExpression {
        r_braced_statements(l_curly_token, r_statement_list(statements), r_curly_token).into()
    }

    fn statement(expression: AnyRExpression) -> AnyRStatement {
        r_expression_statement(expression).build().into()
    }

    fn statement_with_semicolon(
        expression: AnyRExpression,
        semicolon_token: RSyntaxToken,
    ) -> AnyRStatement {
        r_expression_statement(expression)
            .with_semicolon_token(semicolon_token)
            .build()
            .into()
    }

    fn empty(semicolon_token: RSyntaxToken) -> AnyRStatement {
        r_empty_statement(semicolon_token).into()
    }

    fn double(value_token: RSyntaxToken) -> AnyRExpression {
        AnyRExpression::AnyRValue(r_double_value(value_token).into())
    }

    fn identifier(name_token: RSyntaxToken) -> AnyRExpression {
        r_identifier(name_token).into()
    }

    fn call(name_token: RSyntaxToken, value: AnyRExpression) -> AnyRExpression {
        let argument = r_argument().with_value(value).build();
        let arguments = r_call_arguments(
            token(L_PAREN, &[], "(", &[]),
            r_argument_list([argument], []),
            token(R_PAREN, &[], ")", &[]),
        );
        r_call(identifier(name_token), arguments).into()
    }

    /// Format a hand-built `root`
    ///
    /// Checks that `root` was transcribed from `source` correctly, and that the
    /// output is stable when parsed and formatted again.
    fn format(root: RRoot, source: &str) -> anyhow::Result<String> {
        assert_eq!(root.syntax().to_string(), source);

        let output = format_node(RFormatOptions::default(), root.syntax())?
            .print()?
            .into_code();

        // tree-sitter hides `;`, so we can't parse a kept `;` back
        if output.contains(';') {
            return Ok(output);
        }

        let reparsed = air_r_parser::parse(&output, RParserOptions::default()).into_result()?;
        let reformatted = format_node(RFormatOptions::default(), &reparsed)?
            .print()?
            .into_code();
        assert_eq!(output, reformatted);

        Ok(output)
    }

    #[test]
    fn test_semicolons_between_statements() -> anyhow::Result<()> {
        let source = "\
{
  1;;
  2
}";
        let expected = "\
{
  1
  2
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement_with_semicolon(
                    double(token(R_DOUBLE_LITERAL, &[Newline, INDENT], "1", &[])),
                    token(SEMICOLON, &[], ";", &[]),
                ),
                empty(token(SEMICOLON, &[], ";", &[])),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "2",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);
        Ok(())
    }

    #[test]
    fn test_braces_with_only_semicolons() -> anyhow::Result<()> {
        let source = "\
{;}";
        let expected = "\
{}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [empty(token(SEMICOLON, &[], ";", &[]))],
            token(R_CURLY, &[], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{;;}";
        let expected = "\
{}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                empty(token(SEMICOLON, &[], ";", &[])),
                empty(token(SEMICOLON, &[], ";", &[])),
            ],
            token(R_CURLY, &[], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        Ok(())
    }

    #[test]
    fn test_empty_line_before_lone_semicolon_is_kept() -> anyhow::Result<()> {
        let source = "\
{
  1

  ;
  2
}";
        let expected = "\
{
  1

  2
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
                empty(token(SEMICOLON, &[Newline, Newline, INDENT], ";", &[])),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "2",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);
        Ok(())
    }

    #[test]
    fn test_empty_line_before_leading_lone_semicolon_is_removed() -> anyhow::Result<()> {
        let source = "\
{

  ;
  1
}";
        let expected = "\
{
  1
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                empty(token(SEMICOLON, &[Newline, Newline, INDENT], ";", &[])),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);
        Ok(())
    }

    #[test]
    fn test_lone_semicolon_does_not_add_empty_line() -> anyhow::Result<()> {
        let source = "\
{
  1
  ;
  2
}";
        let expected = "\
{
  1
  2
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
                empty(token(SEMICOLON, &[Newline, INDENT], ";", &[])),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "2",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);
        Ok(())
    }

    #[test]
    fn test_comment_before_lone_semicolon() -> anyhow::Result<()> {
        let source = "\
{
  1 # c
  ;
}";
        let expected = "\
{
  1 # c
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[SPACE, Comment("# c")],
                ))),
                empty(token(SEMICOLON, &[Newline, INDENT], ";", &[])),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{1 # c
;}";
        let expected = "\
{
  1 # c
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[],
                    "1",
                    &[SPACE, Comment("# c")],
                ))),
                empty(token(SEMICOLON, &[Newline], ";", &[])),
            ],
            token(R_CURLY, &[], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        Ok(())
    }

    #[test]
    fn test_comment_after_lone_semicolon_trails_previous_statement() -> anyhow::Result<()> {
        let source = "\
{
  1
  ; # c
}";
        let expected = "\
{
  1 # c
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
                empty(token(
                    SEMICOLON,
                    &[Newline, INDENT],
                    ";",
                    &[SPACE, Comment("# c")],
                )),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);
        Ok(())
    }

    #[test]
    fn test_comment_after_lone_semicolon_leads_next_statement() -> anyhow::Result<()> {
        let source = "\
{
  ; # c
  1
}";
        let expected = "\
{
  # c
  1
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                empty(token(
                    SEMICOLON,
                    &[Newline, INDENT],
                    ";",
                    &[SPACE, Comment("# c")],
                )),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);
        Ok(())
    }

    #[test]
    fn test_comment_after_lone_semicolon_dangles() -> anyhow::Result<()> {
        let source = "\
{;# c
}";
        let expected = "\
{
  # c
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [empty(token(SEMICOLON, &[], ";", &[Comment("# c")]))],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);
        Ok(())
    }

    #[test]
    fn test_comments_around_lone_semicolon_dangle() -> anyhow::Result<()> {
        let source = "\
{
  # a
  ; # b
  # c
}";
        let expected = "\
{
  # a
  # b
  # c
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [empty(token(
                SEMICOLON,
                &[Newline, INDENT, Comment("# a"), Newline, INDENT],
                ";",
                &[SPACE, Comment("# b")],
            ))],
            token(
                R_CURLY,
                &[Newline, INDENT, Comment("# c"), Newline],
                "}",
                &[],
            ),
        ))]);
        assert_eq!(format(tree, source)?, expected);
        Ok(())
    }

    #[test]
    fn test_comments_around_lone_semicolon_keep_order() -> anyhow::Result<()> {
        let source = "\
{
  1
  # a
  ; # b
  2
}";
        let expected = "\
{
  1
  # a
  # b
  2
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
                empty(token(
                    SEMICOLON,
                    &[Newline, INDENT, Comment("# a"), Newline, INDENT],
                    ";",
                    &[SPACE, Comment("# b")],
                )),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "2",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);
        Ok(())
    }

    /// Handles every variant of [crate::comments::handle_empty_statement_comment]
    #[test]
    fn test_handle_empty_statement_comment() -> anyhow::Result<()> {
        const TRAILS: Trivia = Comment("# Trails preceding");
        const LEADS: Trivia = Comment("# Leads following");
        const DANGLING: Trivia = Comment("# Dangling on enclosing");

        // `CommentTextPosition::EndOfLine`

        let source = "\
{
  1; ; # Trails preceding
  2
}";
        let expected = "\
{
  1 # Trails preceding
  2
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement_with_semicolon(
                    double(token(R_DOUBLE_LITERAL, &[Newline, INDENT], "1", &[])),
                    token(SEMICOLON, &[], ";", &[]),
                ),
                empty(token(SEMICOLON, &[SPACE], ";", &[SPACE, TRAILS])),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "2",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{
  1; ; ; # Trails preceding
  2
}";
        let expected = "\
{
  1 # Trails preceding
  2
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement_with_semicolon(
                    double(token(R_DOUBLE_LITERAL, &[Newline, INDENT], "1", &[])),
                    token(SEMICOLON, &[], ";", &[]),
                ),
                empty(token(SEMICOLON, &[SPACE], ";", &[])),
                empty(token(SEMICOLON, &[SPACE], ";", &[SPACE, TRAILS])),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "2",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{
  1
  ; ; # Leads following
  2
}";
        let expected = "\
{
  1
  # Leads following
  2
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
                empty(token(SEMICOLON, &[Newline, INDENT], ";", &[])),
                empty(token(SEMICOLON, &[SPACE], ";", &[SPACE, LEADS])),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "2",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{
  1
  ; # Leads following
  2
}";
        let expected = "\
{
  1
  # Leads following
  2
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
                empty(token(SEMICOLON, &[Newline, INDENT], ";", &[SPACE, LEADS])),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "2",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{
  1
  ; # Trails preceding
}";
        let expected = "\
{
  1 # Trails preceding
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
                empty(token(SEMICOLON, &[Newline, INDENT], ";", &[SPACE, TRAILS])),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{
  ; # Leads following
  2
}";
        let expected = "\
{
  # Leads following
  2
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                empty(token(SEMICOLON, &[Newline, INDENT], ";", &[SPACE, LEADS])),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "2",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{
  ; # Dangling on enclosing
}";
        let expected = "\
{
  # Dangling on enclosing
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [empty(token(
                SEMICOLON,
                &[Newline, INDENT],
                ";",
                &[SPACE, DANGLING],
            ))],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        // `CommentTextPosition::OwnLine`

        let source = "\
{
  1
  # Leads following
  ;
  2
}";
        let expected = "\
{
  1
  # Leads following
  2
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
                empty(token(
                    SEMICOLON,
                    &[Newline, INDENT, LEADS, Newline, INDENT],
                    ";",
                    &[],
                )),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "2",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{
  1
  ;
  # Leads following
  2
}";
        let expected = "\
{
  1
  # Leads following
  2
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
                empty(token(SEMICOLON, &[Newline, INDENT], ";", &[])),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT, LEADS, Newline, INDENT],
                    "2",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{
  # Leads following
  ;
  2
}";
        let expected = "\
{
  # Leads following
  2
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                empty(token(
                    SEMICOLON,
                    &[Newline, INDENT, LEADS, Newline, INDENT],
                    ";",
                    &[],
                )),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "2",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{
  ;
  # Leads following
  2
}";
        let expected = "\
{
  # Leads following
  2
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                empty(token(SEMICOLON, &[Newline, INDENT], ";", &[])),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT, LEADS, Newline, INDENT],
                    "2",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{
  1
  # Trails preceding
  ;
}";
        let expected = "\
{
  1
  # Trails preceding
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
                empty(token(
                    SEMICOLON,
                    &[Newline, INDENT, TRAILS, Newline, INDENT],
                    ";",
                    &[],
                )),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{
  1
  ;
  # Trails preceding
}";
        let expected = "\
{
  1
  # Trails preceding
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
                empty(token(SEMICOLON, &[Newline, INDENT], ";", &[])),
            ],
            token(R_CURLY, &[Newline, INDENT, TRAILS, Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{
  # Dangling on enclosing
  ;
}";
        let expected = "\
{
  # Dangling on enclosing
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [empty(token(
                SEMICOLON,
                &[Newline, INDENT, DANGLING, Newline, INDENT],
                ";",
                &[],
            ))],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
{
  ;
  # Dangling on enclosing
}";
        let expected = "\
{
  # Dangling on enclosing
}
";
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [empty(token(SEMICOLON, &[Newline, INDENT], ";", &[]))],
            token(R_CURLY, &[Newline, INDENT, DANGLING, Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);

        Ok(())
    }

    #[test]
    fn test_function_body_comment_skips_lone_semicolon() -> anyhow::Result<()> {
        fn function(body: AnyRExpression) -> AnyRExpression {
            let parameters = r_parameters(
                token(L_PAREN, &[], "(", &[]),
                r_parameter_list([], []),
                token(R_PAREN, &[], ")", &[SPACE, Comment("# c")]),
            );
            r_function_definition(token(FUNCTION_KW, &[], "function", &[]), parameters, body).into()
        }

        let source = "\
function() # c
{
  ;
  1
}";
        let expected = "\
function() {
  # c
  1
}
";
        let tree = root([statement(function(braces(
            token(L_CURLY, &[Newline], "{", &[]),
            [
                empty(token(SEMICOLON, &[Newline, INDENT], ";", &[])),
                statement(double(token(
                    R_DOUBLE_LITERAL,
                    &[Newline, INDENT],
                    "1",
                    &[],
                ))),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        )))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
function() # c
{;}";
        let expected = "\
function() {
  # c
}
";
        let tree = root([statement(function(braces(
            token(L_CURLY, &[Newline], "{", &[]),
            [empty(token(SEMICOLON, &[], ";", &[]))],
            token(R_CURLY, &[], "}", &[]),
        )))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
function() # c
{
  ;
}";
        let expected = "\
function() {
  # c
}
";
        let tree = root([statement(function(braces(
            token(L_CURLY, &[Newline], "{", &[]),
            [empty(token(SEMICOLON, &[Newline, INDENT], ";", &[]))],
            token(R_CURLY, &[Newline], "}", &[]),
        )))]);
        assert_eq!(format(tree, source)?, expected);

        Ok(())
    }

    #[test]
    fn test_trailing_semicolons_keep_value_position() -> anyhow::Result<()> {
        let source = "\
{
  if (a) 1;;
}";
        let expected = "\
{
  if (a) 1
}
";
        let if_statement = r_if_statement(
            token(IF_KW, &[Newline, INDENT], "if", &[SPACE]),
            token(L_PAREN, &[], "(", &[]),
            identifier(token(IDENT, &[], "a", &[])),
            token(R_PAREN, &[], ")", &[SPACE]),
            double(token(R_DOUBLE_LITERAL, &[], "1", &[])),
        )
        .build();
        let tree = root([statement(braces(
            token(L_CURLY, &[], "{", &[]),
            [
                statement_with_semicolon(if_statement.into(), token(SEMICOLON, &[], ";", &[])),
                empty(token(SEMICOLON, &[], ";", &[])),
            ],
            token(R_CURLY, &[Newline], "}", &[]),
        ))]);
        assert_eq!(format(tree, source)?, expected);
        Ok(())
    }

    #[test]
    fn test_curly_curly_removes_semicolon() -> anyhow::Result<()> {
        let source = "\
fn({{ x }; })";
        let expected = "\
fn({{ x }})
";
        let inner = braces(
            token(L_CURLY, &[], "{", &[SPACE]),
            [statement(identifier(token(IDENT, &[], "x", &[SPACE])))],
            token(R_CURLY, &[], "}", &[]),
        );
        let outer = braces(
            token(L_CURLY, &[], "{", &[]),
            [statement_with_semicolon(
                inner,
                token(SEMICOLON, &[], ";", &[SPACE]),
            )],
            token(R_CURLY, &[], "}", &[]),
        );
        let tree = root([statement(call(token(IDENT, &[], "fn", &[]), outer))]);
        assert_eq!(format(tree, source)?, expected);

        let source = "\
fn({{ x; }})";
        let expected = "\
fn({{ x }})
";
        let inner = braces(
            token(L_CURLY, &[], "{", &[SPACE]),
            [statement_with_semicolon(
                identifier(token(IDENT, &[], "x", &[])),
                token(SEMICOLON, &[], ";", &[SPACE]),
            )],
            token(R_CURLY, &[], "}", &[]),
        );
        let outer = braces(
            token(L_CURLY, &[], "{", &[]),
            [statement(inner)],
            token(R_CURLY, &[], "}", &[]),
        );
        let tree = root([statement(call(token(IDENT, &[], "fn", &[]), outer))]);
        assert_eq!(format(tree, source)?, expected);

        Ok(())
    }

    #[test]
    fn test_skip_keeps_semicolon() -> anyhow::Result<()> {
        let source = "\
# fmt: skip
x;";
        let expected = "\
# fmt: skip
x;
";
        let tree = root([statement_with_semicolon(
            identifier(token(IDENT, &[Comment("# fmt: skip"), Newline], "x", &[])),
            token(SEMICOLON, &[], ";", &[]),
        )]);
        assert_eq!(format(tree, source)?, expected);
        Ok(())
    }

    #[test]
    fn test_comment_after_semicolon() -> anyhow::Result<()> {
        let source = "\
1; # c";
        let expected = "\
1 # c
";
        let tree = root([statement_with_semicolon(
            double(token(R_DOUBLE_LITERAL, &[], "1", &[])),
            token(SEMICOLON, &[], ";", &[SPACE, Comment("# c")]),
        )]);
        assert_eq!(format(tree, source)?, expected);
        Ok(())
    }
}
