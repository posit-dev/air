use crate::{RStringValue, inner_string_text};
use biome_rowan::{SyntaxResult, TokenText};

impl RStringValue {
    pub fn inner_string_text(&self) -> SyntaxResult<TokenText> {
        Ok(inner_string_text(&self.value_token()?))
    }
}
