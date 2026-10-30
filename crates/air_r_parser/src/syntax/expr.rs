//! Pratt parser core
//!
//! https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html
//! https://kevinushey.github.io/blog/2016/02/12/top-down-operator-precedence-parsing-with-r/

use crate::parser::RParser;
use crate::syntax::associativity::Associativity;
use crate::syntax::binary::parse_binary_expression;
use crate::syntax::binary::parse_binary_expression_unexpected_binary_equals_assign;
use crate::syntax::binding_power::BindingPower;
use crate::syntax::braced_statements::parse_braced_statements;
use crate::syntax::call::parse_call_expression;
use crate::syntax::call::parse_subset_expression;
use crate::syntax::call::parse_subset2_expression;
use crate::syntax::extract::parse_extract_expression;
use crate::syntax::function::parse_function_definition;
use crate::syntax::if_statement::parse_if_statement;
use crate::syntax::literal::parse_literal_expression;
use crate::syntax::loop_statements::parse_for_statement;
use crate::syntax::loop_statements::parse_repeat_statement;
use crate::syntax::loop_statements::parse_while_statement;
use crate::syntax::namespace::parse_namespace_expression;
use crate::syntax::parenthesized_expression::parse_parenthesized_expression;
use crate::syntax::restrictions::BinaryEqualsAssign;
use crate::syntax::unary::parse_unary_expression;
use air_r_syntax::RSyntaxKind::*;
use air_r_syntax::T;
use biome_parser::Parser;
use biome_parser::ParserProgress;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::Absent;
use biome_parser::parsed_syntax::ParsedSyntax::Present;

/// The Pratt loop
pub(crate) fn parse_expression(
    p: &mut RParser,
    current_binding_power: BindingPower,
    binary_equals_assign: BinaryEqualsAssign,
) -> ParsedSyntax {
    debug_assert!(
        current_binding_power <= BindingPower::HIGHEST,
        "{current_binding_power:?} must not exceed {:?}.",
        BindingPower::HIGHEST
    );

    let mut lhs = match p.cur() {
        kind @ T![?] => {
            parse_unary_expression(p, kind, BindingPower::UNARY_HELP, BinaryEqualsAssign::Allow)
        }
        kind @ T![~] => {
            parse_unary_expression(p, kind, BindingPower::UNARY_TILDE, binary_equals_assign)
        }
        kind @ T![!] => {
            parse_unary_expression(p, kind, BindingPower::UNARY_NOT, binary_equals_assign)
        }
        kind @ (T![+] | T![-]) => parse_unary_expression(
            p,
            kind,
            BindingPower::UNARY_PLUS_MINUS,
            binary_equals_assign,
        ),
        R_INTEGER_LITERAL => parse_literal_expression(p, R_INTEGER_LITERAL, R_INTEGER_VALUE),
        R_DOUBLE_LITERAL => parse_literal_expression(p, R_DOUBLE_LITERAL, R_DOUBLE_VALUE),
        R_COMPLEX_LITERAL => parse_literal_expression(p, R_COMPLEX_LITERAL, R_COMPLEX_VALUE),
        IDENT => parse_literal_expression(p, IDENT, R_IDENTIFIER),
        DOTS => parse_literal_expression(p, DOTS, R_DOTS),
        DOTDOTI => parse_literal_expression(p, DOTDOTI, R_DOT_DOT_I),
        T![next] => parse_literal_expression(p, T![next], R_NEXT_EXPRESSION),
        T![break] => parse_literal_expression(p, T![break], R_BREAK_EXPRESSION),
        T![true] => parse_literal_expression(p, T![true], R_TRUE_EXPRESSION),
        T![false] => parse_literal_expression(p, T![false], R_FALSE_EXPRESSION),
        T![null] => parse_literal_expression(p, T![null], R_NULL_EXPRESSION),
        T![inf] => parse_literal_expression(p, T![inf], R_INF_EXPRESSION),
        T![nan] => parse_literal_expression(p, T![nan], R_NAN_EXPRESSION),
        kind @ (T![na_logical]
        | T![na_integer]
        | T![na_double]
        | T![na_complex]
        | T![na_character]) => parse_literal_expression(p, kind, R_NA_EXPRESSION),
        // Every ERROR_TOKEN already carries a lexer diagnostic, from
        // `unexpected_character()`, `unterminated_string()`, or
        // `unterminated_backtick_identifier()`
        ERROR_TOKEN => parse_literal_expression(p, ERROR_TOKEN, R_BOGUS_EXPRESSION),
        kind @ (R_SINGLE_QUOTED_STRING_LITERAL
        | R_DOUBLE_QUOTED_STRING_LITERAL
        | R_RAW_STRING_LITERAL) => parse_literal_expression(p, kind, R_STRING_VALUE),
        kind @ (T![function] | T!["\\"]) => parse_function_definition(p, kind),
        T![if] => parse_if_statement(p),
        T![for] => parse_for_statement(p),
        T![while] => parse_while_statement(p),
        T![repeat] => parse_repeat_statement(p),
        T!['('] => parse_parenthesized_expression(p),
        T!['{'] => parse_braced_statements(p),
        // Doesn't start an expression
        _ => return Absent,
    };

    let mut progress = ParserProgress::default();

    loop {
        progress.assert_progressing(p);

        if p.scope().newlines_are_significant() && p.has_preceding_line_break() {
            break;
        }

        lhs = match p.cur() {
            T!['('] => parse_call_expression(p, lhs),
            T!['['] => parse_subset_expression(p, lhs),
            T!["[["] => parse_subset2_expression(p, lhs),
            kind @ (T![$] | T![@]) => parse_extract_expression(p, lhs, kind),
            kind @ (T![::] | T![:::]) => parse_namespace_expression(p, lhs, kind),
            kind @ T![?] if BindingPower::BINARY_HELP >= current_binding_power => {
                parse_binary_expression(
                    p,
                    lhs,
                    kind,
                    BindingPower::BINARY_HELP,
                    Associativity::BINARY_HELP,
                    binary_equals_assign,
                )
            }
            kind @ (T![<-] | T![<<-] | T![:=])
                if BindingPower::BINARY_LEFT_ASSIGN >= current_binding_power =>
            {
                parse_binary_expression(
                    p,
                    lhs,
                    kind,
                    BindingPower::BINARY_LEFT_ASSIGN,
                    Associativity::BINARY_LEFT_ASSIGN,
                    binary_equals_assign,
                )
            }
            kind @ T![=] if BindingPower::BINARY_EQUALS_ASSIGN >= current_binding_power => {
                match binary_equals_assign {
                    BinaryEqualsAssign::Allow => parse_binary_expression(
                        p,
                        lhs,
                        kind,
                        BindingPower::BINARY_EQUALS_ASSIGN,
                        Associativity::BINARY_EQUALS_ASSIGN,
                        binary_equals_assign,
                    ),
                    BinaryEqualsAssign::Deny => {
                        parse_binary_expression_unexpected_binary_equals_assign(
                            p,
                            lhs,
                            kind,
                            BindingPower::BINARY_EQUALS_ASSIGN,
                            Associativity::BINARY_EQUALS_ASSIGN,
                            BinaryEqualsAssign::Allow,
                        )
                    }
                }
            }
            kind @ (T![->] | T![->>])
                if BindingPower::BINARY_RIGHT_ASSIGN >= current_binding_power =>
            {
                parse_binary_expression(
                    p,
                    lhs,
                    kind,
                    BindingPower::BINARY_RIGHT_ASSIGN,
                    Associativity::BINARY_RIGHT_ASSIGN,
                    binary_equals_assign,
                )
            }
            kind @ T![~] if BindingPower::BINARY_TILDE >= current_binding_power => {
                parse_binary_expression(
                    p,
                    lhs,
                    kind,
                    BindingPower::BINARY_TILDE,
                    Associativity::BINARY_TILDE,
                    binary_equals_assign,
                )
            }
            kind @ (T![|] | T![||]) if BindingPower::BINARY_OR >= current_binding_power => {
                parse_binary_expression(
                    p,
                    lhs,
                    kind,
                    BindingPower::BINARY_OR,
                    Associativity::BINARY_OR,
                    binary_equals_assign,
                )
            }
            kind @ (T![&] | T![&&]) if BindingPower::BINARY_AND >= current_binding_power => {
                parse_binary_expression(
                    p,
                    lhs,
                    kind,
                    BindingPower::BINARY_AND,
                    Associativity::BINARY_AND,
                    binary_equals_assign,
                )
            }
            kind @ (T![<] | T![<=] | T![>] | T![>=] | T![==] | T![!=])
                if BindingPower::BINARY_COMPARISON >= current_binding_power =>
            {
                parse_binary_expression(
                    p,
                    lhs,
                    kind,
                    BindingPower::BINARY_COMPARISON,
                    Associativity::BINARY_COMPARISON,
                    binary_equals_assign,
                )
            }
            kind @ (T![+] | T![-]) if BindingPower::BINARY_PLUS_MINUS >= current_binding_power => {
                parse_binary_expression(
                    p,
                    lhs,
                    kind,
                    BindingPower::BINARY_PLUS_MINUS,
                    Associativity::BINARY_PLUS_MINUS,
                    binary_equals_assign,
                )
            }
            kind @ (T![*] | T![/])
                if BindingPower::BINARY_MULTIPLY_DIVIDE >= current_binding_power =>
            {
                parse_binary_expression(
                    p,
                    lhs,
                    kind,
                    BindingPower::BINARY_MULTIPLY_DIVIDE,
                    Associativity::BINARY_MULTIPLY_DIVIDE,
                    binary_equals_assign,
                )
            }
            kind @ (SPECIAL | T![|>])
                if BindingPower::BINARY_SPECIAL_OR_PIPE >= current_binding_power =>
            {
                parse_binary_expression(
                    p,
                    lhs,
                    kind,
                    BindingPower::BINARY_SPECIAL_OR_PIPE,
                    Associativity::BINARY_SPECIAL_OR_PIPE,
                    binary_equals_assign,
                )
            }
            kind @ T![:] if BindingPower::BINARY_COLON >= current_binding_power => {
                parse_binary_expression(
                    p,
                    lhs,
                    kind,
                    BindingPower::BINARY_COLON,
                    Associativity::BINARY_COLON,
                    binary_equals_assign,
                )
            }
            kind @ (T![^] | T![**])
                if BindingPower::BINARY_EXPONENTIATE >= current_binding_power =>
            {
                parse_binary_expression(
                    p,
                    lhs,
                    kind,
                    BindingPower::BINARY_EXPONENTIATE,
                    Associativity::BINARY_EXPONENTIATE,
                    binary_equals_assign,
                )
            }
            // No expression to continue, or upcoming one binds looser than
            // current binding power
            _ => break,
        };
    }

    Present(lhs)
}
