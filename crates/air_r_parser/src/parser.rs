use crate::lexer::RReLexContext;
use crate::options::RParserOptions;
use crate::token_source::RTokenSource;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::T;
use biome_parser::ParserContext;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::diagnostic::merge_diagnostics;
use biome_parser::event::Event;
use biome_parser::prelude::*;

pub(crate) struct RParser<'src> {
    context: ParserContext<RSyntaxKind>,
    source: RTokenSource<'src>,
    options: RParserOptions,
    scope: Scope,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum Scope {
    #[default]
    TopLevel,
    Brace,
    Parenthesis,
    Bracket,
    Bracket2,
}

impl Scope {
    /// A line break ends a statement at top level and inside `{`, and means
    /// nothing inside `(`, `[`, or `[[`
    pub(crate) fn newlines_are_significant(self) -> bool {
        match self {
            Self::TopLevel | Self::Brace => true,
            Self::Parenthesis | Self::Bracket | Self::Bracket2 => false,
        }
    }
}

impl<'src> RParser<'src> {
    pub(crate) fn new(source: &'src str, options: RParserOptions) -> Self {
        Self {
            context: ParserContext::default(),
            source: RTokenSource::from_str(source),
            options,
            scope: Scope::default(),
        }
    }

    #[expect(dead_code, reason = "`RParserOptions` is still empty")]
    pub(crate) fn options(&self) -> &RParserOptions {
        &self.options
    }

    /// Runs `f` with `scope` in effect, restoring the previous value after
    pub(crate) fn with_scope<F, R>(&mut self, scope: Scope, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        let saved = std::mem::replace(&mut self.scope, scope);
        let result = f(self);
        self.scope = saved;
        result
    }

    pub(crate) fn scope(&self) -> Scope {
        self.scope
    }

    pub(crate) fn re_lex(&mut self, context: RReLexContext) -> RSyntaxKind {
        self.source_mut().re_lex(context)
    }

    pub(crate) fn finish(self) -> (Vec<Event<RSyntaxKind>>, Vec<Trivia>, Vec<ParseDiagnostic>) {
        let (trivia, lexer_diagnostics) = self.source.finish();
        let (events, parse_diagnostics) = self.context.finish();
        let diagnostics = merge_diagnostics(lexer_diagnostics, parse_diagnostics);
        (events, trivia, diagnostics)
    }
}

impl<'src> Parser for RParser<'src> {
    type Kind = RSyntaxKind;
    type Source = RTokenSource<'src>;

    fn context(&self) -> &ParserContext<Self::Kind> {
        &self.context
    }

    fn context_mut(&mut self) -> &mut ParserContext<Self::Kind> {
        &mut self.context
    }

    fn source(&self) -> &Self::Source {
        &self.source
    }

    fn source_mut(&mut self) -> &mut Self::Source {
        &mut self.source
    }

    /// Overridden so that every bump checks for a potential `]]` re-lex
    fn do_bump(&mut self, kind: Self::Kind) {
        let end = self.cur_range().end();
        self.context_mut().push_token(kind, end);

        if self.context().is_skipping() {
            self.source_mut().skip_as_trivia();
        } else {
            self.source_mut().bump();
        }

        // Joins an adjacent `] ]` into the `]]` the lexer never emits
        if self.scope == Scope::Bracket2 && self.cur() == T![']'] {
            self.re_lex(RReLexContext::JoinBrackets);
        }
    }
}
