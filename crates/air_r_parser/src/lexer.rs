#[cfg(test)]
mod tests;

use crate::diagnostics::unexpected_character;
use crate::diagnostics::unterminated_backtick_identifier;
use crate::diagnostics::unterminated_exponent;
use crate::diagnostics::unterminated_hexadecimal;
use crate::diagnostics::unterminated_hexadecimal_binary_exponent;
use crate::diagnostics::unterminated_special;
use crate::diagnostics::unterminated_string;
use air_r_syntax::RSyntaxKind;
use air_r_syntax::RSyntaxKind::*;
use air_r_syntax::T;
use biome_parser::diagnostic::ParseDiagnostic;
use biome_parser::lexer::LexContext;
use biome_parser::lexer::Lexer;
use biome_parser::lexer::LexerCheckpoint;
use biome_parser::lexer::LexerWithCheckpoint;
use biome_parser::lexer::ReLexer;
use biome_parser::lexer::TokenFlags;
use biome_rowan::SyntaxKind;
use biome_rowan::TextRange;
use biome_rowan::TextSize;
use unicode_ident::is_xid_continue;
use unicode_ident::is_xid_start;

/// Lex context for R
///
/// There aren't any special contexts for R, so we just have a single `Regular`
/// context that everything is lexed in.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub(crate) enum RLexContext {
    #[default]
    Regular,
}

impl LexContext for RLexContext {
    fn is_regular(&self) -> bool {
        // Because we only have `Regular`!
        true
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RStringDelimiter {
    Single,
    Double,
    /// `r"---[...]---"`, closed by the matching bracket, the same number of
    /// hyphens, and the same quote
    Raw {
        quote: u8,
        hyphens: u8,
        bracket: RawStringBracket,
    },
}

impl RStringDelimiter {
    /// The raw string delimiter opening at the start of `text`, or `None` when
    /// none does
    ///
    /// Reads no further than the delimiter itself
    fn try_from_raw_string_open(text: &str) -> Option<Self> {
        let bytes = text.as_bytes();

        if !matches!(bytes.first()?, b'r' | b'R') {
            return None;
        }

        let quote = match bytes.get(1)? {
            quote @ (b'\'' | b'"') => *quote,
            _ => return None,
        };

        let hyphens = bytes[2..].iter().take_while(|byte| **byte == b'-').count();

        // Bail on pathological 256 hyphens
        let hyphens = u8::try_from(hyphens).ok()?;
        let bracket = RawStringBracket::from_open(*bytes.get(2 + usize::from(hyphens))?)?;

        Some(Self::Raw {
            quote,
            hyphens,
            bracket,
        })
    }

    fn kind(self) -> RSyntaxKind {
        match self {
            Self::Single => R_SINGLE_QUOTED_STRING_LITERAL,
            Self::Double => R_DOUBLE_QUOTED_STRING_LITERAL,
            Self::Raw { .. } => R_RAW_STRING_LITERAL,
        }
    }

    /// The string quote character
    fn quote(self) -> u8 {
        match self {
            Self::Single => b'\'',
            Self::Double => b'"',
            Self::Raw { quote, .. } => quote,
        }
    }

    /// The byte length of the opening delimiter
    fn open_length(self) -> usize {
        match self {
            Self::Single | Self::Double => 1,
            // `r`, the quote, the hyphens, and the bracket
            Self::Raw { hyphens, .. } => 3 + usize::from(hyphens),
        }
    }

    /// The byte length of the closing delimiter
    fn close_length(self) -> usize {
        match self {
            Self::Single | Self::Double => 1,
            // The bracket, the hyphens, and the quote
            Self::Raw { hyphens, .. } => 2 + usize::from(hyphens),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawStringBracket {
    Paren,
    Bracket,
    Brace,
    Pipe,
}

impl RawStringBracket {
    fn from_open(byte: u8) -> Option<Self> {
        match byte {
            b'(' => Some(Self::Paren),
            b'[' => Some(Self::Bracket),
            b'{' => Some(Self::Brace),
            b'|' => Some(Self::Pipe),
            _ => None,
        }
    }

    fn close(self) -> u8 {
        match self {
            Self::Paren => b')',
            Self::Bracket => b']',
            Self::Brace => b'}',
            Self::Pipe => b'|',
        }
    }
}

/// The re-lexing requests the parser can make
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RReLexContext {
    /// Re-lex `]` as `]]` when the very next byte is another `]`. A no-op on any
    /// other current token.
    JoinBrackets,
}

impl RLexer<'_> {
    fn re_lex_join_brackets(&mut self) -> RSyntaxKind {
        match self.current_kind {
            T![']'] if self.peek_byte() == Some(b']') => self.eat_bytes(2, T!["]]"]),
            kind => kind,
        }
    }
}

pub(crate) struct RLexer<'src> {
    source: &'src str,

    /// Byte offset of the next token
    position: usize,

    /// Byte offset of the current token. Its range is `current_start..position`.
    current_start: TextSize,

    current_kind: RSyntaxKind,

    current_flags: TokenFlags,

    /// Sticky across `next_token` calls: set by `consume_newline_or_whitespaces`
    /// and cleared only once a non-trivia token has been stamped with it. This
    /// is what lets the lexer report a preceding line break even though
    /// `NEWLINE` is a token in its own right, and it is exactly
    /// what `biome_js_parser` does.
    after_line_break: bool,

    /// Byte length of the BOM, if the source starts with one
    ///
    /// Nothing in R reads this. It exists because [`LexerCheckpoint`] carries
    /// it, and setting it from `consume_potential_bom` costs one line where a
    /// hardcoded `0` would be a lie the next reader has to re-derive.
    unicode_bom_length: usize,

    diagnostics: Vec<ParseDiagnostic>,
}

impl<'src> Lexer<'src> for RLexer<'src> {
    const NEWLINE: Self::Kind = NEWLINE;
    const WHITESPACE: Self::Kind = WHITESPACE;

    type Kind = RSyntaxKind;
    type LexContext = RLexContext;
    type ReLexContext = RReLexContext;

    fn source(&self) -> &'src str {
        self.source
    }

    fn current(&self) -> Self::Kind {
        self.current_kind
    }

    fn current_start(&self) -> TextSize {
        self.current_start
    }

    /// There's only one "Regular" context for R
    fn next_token(&mut self, _context: Self::LexContext) -> Self::Kind {
        self.current_start = self.text_position();
        self.current_flags = TokenFlags::empty();

        let kind = if self.is_eof() { EOF } else { self.lex_token() };

        // Stamped after lexing so that the lex functions are free to set other
        // flags first
        self.current_flags
            .set(TokenFlags::PRECEDING_LINE_BREAK, self.after_line_break);
        self.current_kind = kind;

        // The bit survives across trivia, so `1\n  # c\n  + 2` reports the break
        // on `+` rather than on the whitespace that happens to precede it
        if !kind.is_trivia() {
            self.after_line_break = false;
        }

        debug_assert!(self.current_start <= self.text_position());
        kind
    }

    fn has_preceding_line_break(&self) -> bool {
        self.current_flags.has_preceding_line_break()
    }

    /// R has no token-level unicode escapes
    fn has_unicode_escape(&self) -> bool {
        false
    }

    fn rewind(&mut self, checkpoint: LexerCheckpoint<Self::Kind>) {
        let LexerCheckpoint {
            position,
            current_start,
            current_kind,
            current_flags,
            after_line_break,
            unicode_bom_length,
            diagnostics_pos,
        } = checkpoint;

        self.position = u32::from(position) as usize;
        self.current_start = current_start;
        self.current_kind = current_kind;
        self.current_flags = current_flags;
        self.after_line_break = after_line_break;
        self.unicode_bom_length = unicode_bom_length;
        self.diagnostics.truncate(diagnostics_pos as usize);
    }

    fn finish(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    fn current_flags(&self) -> TokenFlags {
        self.current_flags
    }

    fn position(&self) -> usize {
        self.position
    }

    fn push_diagnostic(&mut self, diagnostic: ParseDiagnostic) {
        self.diagnostics.push(diagnostic);
    }

    fn advance(&mut self, n: usize) {
        self.position += n;
    }

    fn advance_char_unchecked(&mut self) {
        let char = self.current_char_unchecked();
        self.position += char.len_utf8();
    }

    /// Overridden to update `after_line_break`. The default cannot since this is
    /// a custom field. `biome_js_parser` overrides it for the same reason.
    ///
    /// Inheriting the trait's `consume_newline` also gets `\r\n` treated as a
    /// single newline for free.
    fn consume_newline_or_whitespaces(&mut self) -> Self::Kind {
        if self.consume_newline() {
            self.after_line_break = true;
            NEWLINE
        } else {
            self.consume_whitespaces();
            WHITESPACE
        }
    }

    /// R's whitespace is ASCII: space, tab, vertical tab, and form feed
    ///
    /// The trait's body is written for CSS and diagnoses vertical tab and form
    /// feed, which are ordinary whitespace here.
    fn consume_whitespaces(&mut self) {
        while let Some(b'\t' | b' ' | b'\x0B' | b'\x0C') = self.current_byte() {
            self.advance(1);
        }
    }
}

impl<'src> ReLexer<'src> for RLexer<'src> {
    fn re_lex(&mut self, context: Self::ReLexContext) -> Self::Kind {
        let old_position = self.position;
        self.position = u32::from(self.current_start) as usize;

        let new_kind = match context {
            RReLexContext::JoinBrackets => self.re_lex_join_brackets(),
        };

        if self.current_kind == new_kind {
            // Nothing was re-lexed. Return the existing token untouched.
            self.position = old_position;
        } else {
            self.current_kind = new_kind;
        }

        new_kind
    }
}

impl<'src> LexerWithCheckpoint<'src> for RLexer<'src> {
    fn checkpoint(&self) -> LexerCheckpoint<Self::Kind> {
        LexerCheckpoint {
            position: self.text_position(),
            current_start: self.current_start,
            current_kind: self.current_kind,
            current_flags: self.current_flags,
            after_line_break: self.after_line_break,
            unicode_bom_length: self.unicode_bom_length,
            diagnostics_pos: self.diagnostics.len() as u32,
        }
    }
}

impl<'src> RLexer<'src> {
    pub(crate) fn from_str(source: &'src str) -> Self {
        Self {
            source,
            position: 0,
            current_start: TextSize::from(0),
            current_kind: TOMBSTONE,
            current_flags: TokenFlags::empty(),
            after_line_break: false,
            unicode_bom_length: 0,
            diagnostics: Vec::new(),
        }
    }

    fn lex_token(&mut self) -> RSyntaxKind {
        debug_assert!(!self.is_eof());

        match self.current_byte_unchecked() {
            b'\n' | b'\r' | b'\t' | b' ' | b'\x0B' | b'\x0C' => {
                self.consume_newline_or_whitespaces()
            }

            b'#' => self.consume_comment(),

            b'"' => self.consume_string(RStringDelimiter::Double),
            b'\'' => self.consume_string(RStringDelimiter::Single),
            b'`' => self.consume_backtick_identifier(),

            // Ahead of the digit arm, which would otherwise take the `0`
            b'0' if matches!(self.peek_byte(), Some(b'x' | b'X')) => self.consume_hexadecimal(),
            b'0'..=b'9' => self.consume_number(),

            // `.` is overloaded four ways: `...`, `..1`, `.5`, and `.foo`
            b'.' => self.consume_dot(),

            b'%' => self.consume_special(),

            // Raw strings share this arm with every other name
            // rather than taking one of their own, which would split the letter
            // range and cost the whole match its jump table for `s` onwards.
            b'_' | b'a'..=b'z' | b'A'..=b'Z' => {
                match RStringDelimiter::try_from_raw_string_open(&self.source[self.position..]) {
                    Some(delimiter) => self.consume_string(delimiter),
                    None => self.consume_identifier_or_keyword(),
                }
            }

            b';' => self.eat_byte(T![;]),
            b',' => self.eat_byte(T![,]),
            b'{' => self.eat_byte(T!['{']),
            b'}' => self.eat_byte(T!['}']),
            b'(' => self.eat_byte(T!['(']),
            b')' => self.eat_byte(T![')']),
            b'?' => self.eat_byte(T![?]),
            b'~' => self.eat_byte(T![~]),
            b'+' => self.eat_byte(T![+]),
            b'/' => self.eat_byte(T![/]),
            b'^' => self.eat_byte(T![^]),
            b'$' => self.eat_byte(T![$]),
            b'@' => self.eat_byte(T![@]),
            b'\\' => self.eat_byte(T!["\\"]),

            // Lexer emits `]` but never `]]`. Only the parser can decide if we
            // are in a [`RReLexContext::JoinBrackets`] context or not, at which
            // point re-lexing will join contiguous brackets.
            b']' => self.eat_byte(T![']']),

            // `[[` is greedy in R's own `gram.y` and in tree-sitter-r's scanner.
            // Two adjacent opening brackets are always a subset2.
            b'[' => match self.peek_byte() {
                Some(b'[') => self.eat_bytes(2, T!["[["]),
                _ => self.eat_byte(T!['[']),
            },
            b'<' => match (self.peek_byte(), self.byte_at(2)) {
                (Some(b'<'), Some(b'-')) => self.eat_bytes(3, T![<<-]),
                (Some(b'-'), _) => self.eat_bytes(2, T![<-]),
                (Some(b'='), _) => self.eat_bytes(2, T![<=]),
                _ => self.eat_byte(T![<]),
            },
            b'-' => match (self.peek_byte(), self.byte_at(2)) {
                (Some(b'>'), Some(b'>')) => self.eat_bytes(3, T![->>]),
                (Some(b'>'), _) => self.eat_bytes(2, T![->]),
                _ => self.eat_byte(T![-]),
            },
            b':' => match (self.peek_byte(), self.byte_at(2)) {
                (Some(b':'), Some(b':')) => self.eat_bytes(3, T![:::]),
                (Some(b':'), _) => self.eat_bytes(2, T![::]),
                (Some(b'='), _) => self.eat_bytes(2, T![:=]),
                _ => self.eat_byte(T![:]),
            },
            b'>' => match self.peek_byte() {
                Some(b'=') => self.eat_bytes(2, T![>=]),
                _ => self.eat_byte(T![>]),
            },
            b'=' => match self.peek_byte() {
                Some(b'=') => self.eat_bytes(2, T![==]),
                _ => self.eat_byte(T![=]),
            },
            b'!' => match self.peek_byte() {
                Some(b'=') => self.eat_bytes(2, T![!=]),
                _ => self.eat_byte(T![!]),
            },
            b'&' => match self.peek_byte() {
                Some(b'&') => self.eat_bytes(2, T![&&]),
                _ => self.eat_byte(T![&]),
            },
            b'|' => match self.peek_byte() {
                Some(b'|') => self.eat_bytes(2, T![||]),
                Some(b'>') => self.eat_bytes(2, T![|>]),
                _ => self.eat_byte(T![|]),
            },
            b'*' => match self.peek_byte() {
                Some(b'*') => self.eat_bytes(2, T![**]),
                _ => self.eat_byte(T![*]),
            },

            byte if byte >= 128 => self.consume_unicode(),

            _ => self.consume_unexpected(),
        }
    }

    fn current_byte_unchecked(&self) -> u8 {
        debug_assert!(!self.is_eof());
        self.source.as_bytes()[self.position]
    }

    fn eat_byte(&mut self, kind: RSyntaxKind) -> RSyntaxKind {
        self.advance(1);
        kind
    }

    fn eat_bytes(&mut self, n: usize, kind: RSyntaxKind) -> RSyntaxKind {
        self.advance(n);
        kind
    }

    /// `#` to the end of the line, exclusive of the line break itself
    fn consume_comment(&mut self) -> RSyntaxKind {
        self.assert_byte(b'#');
        self.advance(1);

        // We're just scanning for the next `\n` or `\r`, so we don't need to worry about multi
        // byte UTF-8 characters here. This is faster than calling `advance_byte_or_char()` in a loop.
        let rest = &self.source.as_bytes()[self.position..];
        let len = rest
            .iter()
            .position(|byte| matches!(byte, b'\n' | b'\r'))
            .unwrap_or(rest.len());
        self.advance(len);

        COMMENT
    }

    // -----------------------------------------------------------------------
    // Identifiers and keywords

    /// Consumes an identifier mimicking `[\p{XID_Start}_][\p{XID_Continue}.]*`.
    /// If the consumed identifier maps to a keyword, that is returned instead.
    fn consume_identifier_or_keyword(&mut self) -> RSyntaxKind {
        let start = self.position;
        // `XID_Start` or `_`
        self.advance_char_unchecked();
        self.consume_xid_continue_or_dot();

        Self::keyword_kind(&self.source[start..self.position]).unwrap_or(IDENT)
    }

    /// `[\p{XID_Continue}.]*`
    fn consume_xid_continue_or_dot(&mut self) {
        loop {
            match self.current_byte() {
                Some(b'.' | b'_' | b'0'..=b'9' | b'a'..=b'z' | b'A'..=b'Z') => self.advance(1),
                Some(byte) if byte >= 128 => {
                    if !is_xid_continue(self.current_char_unchecked()) {
                        break;
                    }
                    self.advance_char_unchecked();
                }
                _ => break,
            }
        }
    }

    /// R's keywords and constants, spelled as they appear in source
    fn keyword_kind(text: &str) -> Option<RSyntaxKind> {
        Some(match text {
            "function" => T![function],
            "for" => T![for],
            "in" => T![in],
            "while" => T![while],
            "repeat" => T![repeat],
            "if" => T![if],
            "else" => T![else],
            "next" => T![next],
            "break" => T![break],
            "TRUE" => T![true],
            "FALSE" => T![false],
            "NULL" => T![null],
            "Inf" => T![inf],
            "NaN" => T![nan],
            "NA" => T![na_logical],
            "NA_integer_" => T![na_integer],
            "NA_real_" => T![na_double],
            "NA_complex_" => T![na_complex],
            "NA_character_" => T![na_character],
            _ => return None,
        })
    }

    fn consume_backtick_identifier(&mut self) -> RSyntaxKind {
        self.assert_byte(b'`');
        let open = TextRange::at(self.text_position(), TextSize::from(1));
        self.advance(1);

        while let Some(byte) = self.current_byte() {
            match byte {
                b'`' => {
                    self.advance(1);
                    return IDENT;
                }
                b'\\' => {
                    self.advance(1);
                    if let Some(escaped) = self.current_byte() {
                        self.advance_byte_or_char(escaped);
                    }
                }
                byte => self.advance_byte_or_char(byte),
            }
        }

        let eof = TextRange::empty(self.text_position());
        self.push_diagnostic(unterminated_backtick_identifier(open, eof));
        ERROR_TOKEN
    }

    /// The `byte >= 128` arm, which is also where the BOM is resolved
    fn consume_unicode(&mut self) -> RSyntaxKind {
        // A BOM is only a BOM at offset 0. Elsewhere it falls through as unexpected.
        if self.position == 0
            && let Some((bom, length)) = self.consume_potential_bom(UNICODE_BOM)
        {
            self.unicode_bom_length = length;
            return bom;
        }

        if is_xid_start(self.current_char_unchecked()) {
            self.consume_identifier_or_keyword()
        } else {
            self.consume_unexpected()
        }
    }

    // -----------------------------------------------------------------------
    // Dots

    /// `...`, `..1`, `.5`, and `.foo` all start with a `.`
    ///
    /// R decides between a number and a name on the single character after the
    /// `.` (`typeofnext()` in `src/main/gram.y`), then consumes one whole name
    /// and classifies its text. `...` and `..i` are not tokens of their own
    /// there — `"..."` is a row in R's keyword table, and `..1` is an ordinary
    /// symbol.
    fn consume_dot(&mut self) -> RSyntaxKind {
        self.assert_byte(b'.');

        // A digit after the `.` always makes a number, never a name, otherwise
        // `.1i` and `.1L` lex as identifiers (r-lib/tree-sitter-r#190).
        if matches!(self.peek_byte(), Some(b'0'..=b'9')) {
            return self.consume_number();
        }

        let start = self.position;
        self.advance(1);
        self.consume_xid_continue_or_dot();

        Self::dot_kind(&self.source[start..self.position])
    }

    /// The two names the parser has to tell apart from any other identifier
    fn dot_kind(text: &str) -> RSyntaxKind {
        match text.as_bytes() {
            b"..." => T![...],
            [b'.', b'.', digits @ ..]
                if !digits.is_empty() && digits.iter().all(u8::is_ascii_digit) =>
            {
                DOTDOTI
            }
            _ => IDENT,
        }
    }

    // -----------------------------------------------------------------------
    // Numbers

    /// `(?:(?:\d+(?:\.\d*)?)|(?:\.\d+))(?:[eE][+-]?\d+)?`
    fn consume_number(&mut self) -> RSyntaxKind {
        let start = self.text_position();
        self.consume_digits();

        if self.current_byte() == Some(b'.') {
            self.advance(1);
            self.consume_digits();
        }

        if matches!(self.current_byte(), Some(b'e' | b'E')) {
            self.advance(1);

            if !self.consume_exponent() {
                let range = TextRange::new(start, self.text_position());
                self.push_diagnostic(unterminated_exponent(range));
            }
        }

        self.consume_suffix()
    }

    /// `0[xX](([0-9a-fA-F]+(\.[0-9a-fA-F]*)?)|(\.[0-9a-fA-F]*))([pP][+-]?[0-9]+)?`
    fn consume_hexadecimal(&mut self) -> RSyntaxKind {
        let start = self.text_position();

        // The `0x`
        self.advance(2);

        let has_body = self
            .current_byte()
            .is_some_and(|byte| byte == b'.' || byte.is_ascii_hexdigit());

        if !has_body {
            let range = TextRange::new(start, self.text_position());
            self.push_diagnostic(unterminated_hexadecimal(range));
            return self.consume_suffix();
        }

        self.consume_hex_digits();

        if self.current_byte() == Some(b'.') {
            self.advance(1);
            self.consume_hex_digits();
        }

        if matches!(self.current_byte(), Some(b'p' | b'P')) {
            self.advance(1);

            if !self.consume_exponent() {
                let range = TextRange::new(start, self.text_position());
                self.push_diagnostic(unterminated_hexadecimal_binary_exponent(range));
            }
        }

        self.consume_suffix()
    }

    /// The `L` and `i` that turn the literal before them into an integer or a
    /// complex
    fn consume_suffix(&mut self) -> RSyntaxKind {
        match self.current_byte() {
            Some(b'L') => self.eat_byte(R_INTEGER_LITERAL),
            Some(b'i') => self.eat_byte(R_COMPLEX_LITERAL),
            _ => R_DOUBLE_LITERAL,
        }
    }

    /// An optional sign and the exponent's digits, reporting whether the digits
    /// were there
    fn consume_exponent(&mut self) -> bool {
        if matches!(self.current_byte(), Some(b'+' | b'-')) {
            self.advance(1);
        }

        self.consume_digits()
    }

    /// Whether any digit was consumed
    fn consume_digits(&mut self) -> bool {
        let start = self.position;
        while matches!(self.current_byte(), Some(byte) if byte.is_ascii_digit()) {
            self.advance(1);
        }
        self.position != start
    }

    /// Whether any digit was consumed
    fn consume_hex_digits(&mut self) -> bool {
        let start = self.position;
        while matches!(self.current_byte(), Some(byte) if byte.is_ascii_hexdigit()) {
            self.advance(1);
        }
        self.position != start
    }

    // -----------------------------------------------------------------------
    // Special operators

    /// `/%[^%\n]*%/`, i.e. `%%`, `%/%`, `%in%`, `%o%`, and friends
    ///
    /// Unlike strings and backticked names, a `%...%` may not span a newline.
    /// The partial `%foo` is kept as a `SPECIAL` carrying the diagnostic.
    fn consume_special(&mut self) -> RSyntaxKind {
        self.assert_byte(b'%');
        let open = TextRange::at(self.text_position(), TextSize::from(1));

        // The `%`
        self.advance(1);

        while let Some(byte) = self.current_byte() {
            match byte {
                b'%' => {
                    self.advance(1);
                    return SPECIAL;
                }
                b'\n' | b'\r' => break,
                byte => self.advance_byte_or_char(byte),
            }
        }

        self.push_diagnostic(unterminated_special(open));
        SPECIAL
    }

    // -----------------------------------------------------------------------
    // Strings

    fn consume_string(&mut self, delimiter: RStringDelimiter) -> RSyntaxKind {
        let open = TextRange::at(
            self.text_position(),
            TextSize::from(delimiter.open_length() as u32),
        );
        self.advance(delimiter.open_length());

        let terminated = match delimiter {
            RStringDelimiter::Single | RStringDelimiter::Double => {
                self.consume_quoted_string_content(delimiter.quote())
            }
            RStringDelimiter::Raw {
                quote,
                hyphens,
                bracket,
            } => self.consume_raw_string_content(quote, hyphens, bracket),
        };

        if !terminated {
            let eof = TextRange::empty(self.text_position());
            self.push_diagnostic(unterminated_string(open, eof));
            return ERROR_TOKEN;
        }

        self.advance(delimiter.close_length());
        delimiter.kind()
    }

    /// Everything up to the closing quote
    fn consume_quoted_string_content(&mut self, quote: u8) -> bool {
        while let Some(byte) = self.current_byte() {
            match byte {
                b'\\' => {
                    self.advance(1);
                    if let Some(escaped) = self.current_byte() {
                        self.advance_byte_or_char(escaped);
                    }
                }
                byte if byte == quote => return true,
                byte => self.advance_byte_or_char(byte),
            }
        }

        // Unterminated string
        false
    }

    /// Everything up to the closing sequence
    ///
    /// The subtle part is the closing bracket that turns out not to start one.
    /// It belongs to the body, and the scan carries on from the byte after it,
    /// or `r"(())"` and `r"-())-"` never find their close
    /// (r-lib/tree-sitter-r#162).
    fn consume_raw_string_content(
        &mut self,
        quote: u8,
        hyphens: u8,
        bracket: RawStringBracket,
    ) -> bool {
        while let Some(byte) = self.current_byte() {
            if self.at_raw_string_close(quote, hyphens, bracket) {
                return true;
            }
            self.advance_byte_or_char(byte);
        }

        // Unterminated string
        false
    }

    /// Whether a whole closing sequence — the bracket, the hyphens, and the
    /// quote — starts at the current position
    fn at_raw_string_close(&self, quote: u8, hyphens: u8, bracket: RawStringBracket) -> bool {
        if self.current_byte() != Some(bracket.close()) {
            return false;
        }

        let hyphens = usize::from(hyphens);
        (1..=hyphens).all(|offset| self.byte_at(offset) == Some(b'-'))
            && self.byte_at(hyphens + 1) == Some(quote)
    }

    // -----------------------------------------------------------------------
    // Errors

    /// A character that starts no token
    ///
    /// Exactly one character is consumed and passed on as `ERROR_TOKEN`, which
    /// is what `biome_js_parser` also does.
    fn consume_unexpected(&mut self) -> RSyntaxKind {
        let start = self.text_position();
        let character = self.current_char_unchecked();
        self.advance(character.len_utf8());

        let range = TextRange::new(start, self.text_position());
        self.push_diagnostic(unexpected_character(range, character));
        ERROR_TOKEN
    }
}
