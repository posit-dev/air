use crate::prelude::*;
use air_r_syntax::RParameter;
use air_r_syntax::RParameterFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRParameter;
impl FormatNodeRule<RParameter> for FormatRParameter {
    fn fmt_fields(&self, node: &RParameter, f: &mut RFormatter) -> FormatResult<()> {
        let RParameterFields { name, default } = node.as_fields();

        write!(f, [name.format()])?;

        if let Some(default) = default {
            write!(f, [space(), default.format()])?;
        }

        Ok(())
    }
}
