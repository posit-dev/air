use crate::diagnostics;
use crate::parser::RParser;
use crate::parser::Scope;
use crate::syntax::statement_list::parse_statement_list;
use air_r_syntax::RSyntaxKind::R_BRACED_STATEMENTS;
use air_r_syntax::T;
use biome_parser::CompletedMarker;
use biome_parser::Parser;

pub(crate) fn parse_braced_statements(p: &mut RParser) -> CompletedMarker {
    let m = p.start();
    let open_range = p.cur_range();

    p.with_scope(Scope::Brace, |p| {
        p.bump(T!['{']);
        parse_statement_list(p, T!['}']);
    });

    if !p.eat(T!['}']) {
        p.error(diagnostics::unterminated_scope(
            p,
            T!['{'],
            open_range,
            T!['}'],
            p.cur_range(),
        ));
    }

    m.complete(p, R_BRACED_STATEMENTS)
}
