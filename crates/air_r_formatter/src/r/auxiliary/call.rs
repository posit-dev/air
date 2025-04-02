use crate::is_suppressed_by_comment;
use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use air_r_syntax::AnyRSelector;
use air_r_syntax::RCall;
use air_r_syntax::RCallFields;
use air_r_syntax::RIdentifier;
use biome_formatter::write;
use biome_rowan::SyntaxResult;
use settings::Skip;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRCall;
impl FormatNodeRule<RCall> for FormatRCall {
    fn fmt_fields(&self, node: &RCall, f: &mut RFormatter) -> FormatResult<()> {
        let RCallFields {
            function,
            arguments,
        } = node.as_fields();

        write!(f, [function.format(), arguments.format()])
    }

    fn is_suppressed(&self, node: &RCall, f: &RFormatter) -> bool {
        is_suppressed_by_comment(node, f) || is_suppressed_by_skip(node, f).unwrap_or(false)
    }
}

#[inline]
fn is_suppressed_by_skip(node: &RCall, f: &RFormatter) -> SyntaxResult<bool> {
    let Some(skip) = f.options().skip() else {
        // Nothing to do if user didn't supply any skip functions
        return Ok(false);
    };

    Ok(match node.function()? {
        AnyRExpression::RIdentifier(node) => is_skip(node, skip)?,
        AnyRExpression::RNamespaceExpression(node) => match node.right()? {
            AnyRSelector::RIdentifier(node) => is_skip(node, skip)?,
            AnyRSelector::RDotDotI(_) => false,
            AnyRSelector::RDots(_) => false,
            AnyRSelector::RStringValue(_) => false,
        },
        _ => false,
    })
}

#[inline]
fn is_skip(node: RIdentifier, skip: &Skip) -> SyntaxResult<bool> {
    let node = node.name_token()?;
    Ok(skip.contains(node.text_trimmed()))
}
