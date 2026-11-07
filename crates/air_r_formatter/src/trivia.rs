use air_r_syntax::RSyntaxToken;
use biome_formatter::Argument;
use biome_formatter::Format;
use biome_formatter::FormatResult;
use biome_formatter::formatter::Formatter;
use biome_formatter::trivia::FormatToken;

use crate::FormatRSyntaxToken;
use crate::RFormatter;
use crate::prelude::RFormatContext;

pub(crate) struct FormatRemoved<'a> {
    token: &'a RSyntaxToken,
}

pub(crate) fn format_removed(token: &RSyntaxToken) -> FormatRemoved<'_> {
    FormatRemoved { token }
}

impl<'a> Format<RFormatContext> for FormatRemoved<'a> {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        FormatRSyntaxToken.format_removed(self.token, f)
    }
}

pub(crate) struct FormatReplaced<'a> {
    token: &'a RSyntaxToken,
    content: Argument<'a, RFormatContext>,
}

pub(crate) fn format_replaced<'a>(
    token: &'a RSyntaxToken,
    content: &'a impl Format<RFormatContext>,
) -> FormatReplaced<'a> {
    FormatReplaced {
        token,
        content: Argument::new(content),
    }
}

impl<'a> Format<RFormatContext> for FormatReplaced<'a> {
    fn fmt(&self, f: &mut Formatter<RFormatContext>) -> FormatResult<()> {
        FormatRSyntaxToken.format_replaced(self.token, &self.content, f)
    }
}

pub(crate) fn on_skipped(token: &RSyntaxToken, f: &mut RFormatter) -> FormatResult<()> {
    FormatRSyntaxToken.format_skipped_token_trivia(token, f)
}

pub(crate) fn on_removed(token: &RSyntaxToken, f: &mut RFormatter) -> FormatResult<()> {
    FormatRSyntaxToken.format_removed(token, f)
}
