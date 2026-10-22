use crate::diagnostics;
use crate::parser::RParser;
use crate::parser::Scope;
use crate::syntax::body::parse_body_expression;
use crate::syntax::expr::parse_expression;
use air_r_syntax::RSyntaxKind::R_ELSE_CLAUSE;
use air_r_syntax::RSyntaxKind::R_IF_STATEMENT;
use air_r_syntax::T;
use biome_parser::CompletedMarker;
use biome_parser::Parser;

pub(crate) fn parse_if_statement(p: &mut RParser) -> CompletedMarker {
    let m = p.start();
    p.bump(T![if]);

    p.with_scope(Scope::Parenthesis, |p| {
        p.expect(T!['(']);
        parse_expression(p).or_add_diagnostic(p, diagnostics::expected_condition);
    });
    p.expect(T![')']);

    // The consequence and the alternative have the same limitation as a loop
    // body: treated like any other expression, but `?` comes before them in the
    // precedence table. In gram.y a loop body is left associative and an `if`
    // body is right associative one rank up, but they shake out the same
    parse_body_expression(p).or_add_diagnostic(p, diagnostics::expected_if_body);
    parse_else_clause(p);

    m.complete(p, R_IF_STATEMENT)
}

fn parse_else_clause(p: &mut RParser) {
    if !p.at(T![else]) {
        return;
    }

    // A newline before `else` ends the `if` at top level and nowhere else
    if p.scope() == Scope::TopLevel && p.has_preceding_line_break() {
        return;
    }

    let m = p.start();
    p.bump(T![else]);
    parse_body_expression(p).or_add_diagnostic(p, diagnostics::expected_else_body);
    m.complete(p, R_ELSE_CLAUSE);
}
