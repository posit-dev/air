use crate::prelude::*;
use crate::string_content::FormatStringContentToken;
use air_r_syntax::RStringValue;
use air_r_syntax::RStringValueFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRStringValue;
impl FormatNodeRule<RStringValue> for FormatRStringValue {
    fn fmt_fields(&self, node: &RStringValue, f: &mut RFormatter) -> FormatResult<()> {
        let RStringValueFields {
            open_token,
            content_token,
            close_token,
        } = node.as_fields();

        write!(f, [open_token.format()])?;

        if let Some(content_token) = content_token {
            write!(f, [FormatStringContentToken::new(&content_token)])?;
        }

        write!(f, [close_token.format()])?;

        Ok(())
    }
}
