use crate::parser::RParser;
use crate::syntax::literal::parse_literal_expression;
use air_r_syntax::RSyntaxKind::DOTDOTI;
use air_r_syntax::RSyntaxKind::DOTS;
use air_r_syntax::RSyntaxKind::IDENT;
use air_r_syntax::RSyntaxKind::R_DOT_DOT_I;
use air_r_syntax::RSyntaxKind::R_DOTS;
use air_r_syntax::RSyntaxKind::R_DOUBLE_QUOTED_STRING_LITERAL;
use air_r_syntax::RSyntaxKind::R_IDENTIFIER;
use air_r_syntax::RSyntaxKind::R_RAW_STRING_LITERAL;
use air_r_syntax::RSyntaxKind::R_SINGLE_QUOTED_STRING_LITERAL;
use air_r_syntax::RSyntaxKind::R_STRING_VALUE;
use biome_parser::Parser;
use biome_parser::parsed_syntax::ParsedSyntax;
use biome_parser::parsed_syntax::ParsedSyntax::Absent;
use biome_parser::parsed_syntax::ParsedSyntax::Present;

/// identifier | string | `...` | `..i`
///
/// Used by [crate::syntax::extract::parse_extract_expression()] and
/// [crate::syntax::namespace::parse_namespace_expression()]
pub(crate) fn parse_selector(p: &mut RParser) -> ParsedSyntax {
    match p.cur() {
        IDENT => Present(parse_literal_expression(p, IDENT, R_IDENTIFIER)),
        DOTS => Present(parse_literal_expression(p, DOTS, R_DOTS)),
        DOTDOTI => Present(parse_literal_expression(p, DOTDOTI, R_DOT_DOT_I)),
        kind @ (R_SINGLE_QUOTED_STRING_LITERAL
        | R_DOUBLE_QUOTED_STRING_LITERAL
        | R_RAW_STRING_LITERAL) => Present(parse_literal_expression(p, kind, R_STRING_VALUE)),
        _ => Absent,
    }
}
