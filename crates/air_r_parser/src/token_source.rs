#[cfg(test)]
mod tests;

use crate::lexer::RLexContext;
use crate::lexer::RLexer;
use crate::lexer::RReLexContext;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::RSyntaxKind::EOF;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::BufferedLexer;
use biome_parser::prelude::TokenSource;
use biome_parser::prelude::Trivia;
use biome_parser::token_source::TokenSourceWithBufferedLexer;
use biome_rowan::TextRange;
use biome_rowan::TriviaPieceKind;

pub(crate) struct RTokenSource<'src> {
    lexer: BufferedLexer<RSyntaxKind, RLexer<'src>>,

    /// The trivia the CST is rebuilt from, in source order
    trivia: Vec<Trivia>,
}

impl<'src> RTokenSource<'src> {
    pub(crate) fn from_str(source: &'src str) -> Self {
        let lexer = BufferedLexer::new(RLexer::from_str(source));

        let mut source = Self {
            lexer,
            trivia: Vec::new(),
        };

        source.next_non_trivia_token(true);
        source
    }

    /// Advances to the next non-trivia token, collecting the trivia it skips
    ///
    /// Trivia trails the token before it until a newline is seen, and leads the
    /// token after it from the newline onward. Before the first token of the
    /// file, everything leads. That aligns with Biome's parsers and is both
    /// very logical and very practical because it doesn't require any
    /// lookahead.
    fn next_non_trivia_token(&mut self, first: bool) {
        let mut trailing = !first;

        loop {
            let kind = self.lexer.next_token(RLexContext::Regular);

            let Ok(trivia_kind) = TriviaPieceKind::try_from(kind) else {
                break;
            };

            // We've now seen a newline, so this newline and everything after it
            // is now considered leading trivia of the next token
            if trivia_kind.is_newline() {
                trailing = false;
            }

            self.trivia
                .push(Trivia::new(trivia_kind, self.current_range(), trailing));
        }
    }

    /// Re-lexes the current token. Buffered lookahead is cleared if the kind
    /// changes.
    pub(crate) fn re_lex(&mut self, context: RReLexContext) -> RSyntaxKind {
        self.lexer.re_lex(context)
    }
}

impl TokenSource for RTokenSource<'_> {
    type Kind = RSyntaxKind;

    fn current(&self) -> Self::Kind {
        self.lexer.current()
    }

    fn current_range(&self) -> TextRange {
        self.lexer.current_range()
    }

    fn text(&self) -> &str {
        self.lexer.source()
    }

    fn has_preceding_line_break(&self) -> bool {
        self.lexer.has_preceding_line_break()
    }

    fn bump(&mut self) {
        if self.current() != EOF {
            self.next_non_trivia_token(false);
        }
    }

    fn skip_as_trivia(&mut self) {
        if self.current() != EOF {
            self.trivia.push(Trivia::new(
                TriviaPieceKind::Skipped,
                self.current_range(),
                false,
            ));

            self.next_non_trivia_token(true);
        }
    }

    fn finish(self) -> (Vec<Trivia>, Vec<ParseDiagnostic>) {
        (self.trivia, self.lexer.finish())
    }
}

impl<'src> TokenSourceWithBufferedLexer<RLexer<'src>> for RTokenSource<'src> {
    fn lexer(&mut self) -> &mut BufferedLexer<RSyntaxKind, RLexer<'src>> {
        &mut self.lexer
    }
}
