use crate::parser::RParser;
use crate::syntax::statement_list::parse_statement_list;
use air_r_syntax::RSyntaxKind::R_ROOT;
use air_r_syntax::T;
use biome_parser::Parser;

pub(crate) fn parse_root(p: &mut RParser) {
    let m = p.start();
    p.eat(T![UNICODE_BOM]);
    parse_statement_list(p, T![EOF]);
    p.expect(T![EOF]);
    m.complete(p, R_ROOT);
}
