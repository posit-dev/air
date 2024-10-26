use crate::{inner_string_text, RStringValue};
use biome_rowan::{SyntaxResult, TokenText};

impl RStringValue {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}
