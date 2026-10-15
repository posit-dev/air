use crate::prelude::*;
use air_r_syntax::AnyRExpression;
use air_r_syntax::RArgument;
use air_r_syntax::RArgumentFields;
use air_r_syntax::RArgumentList;
use air_r_syntax::RCall;
use air_r_syntax::RCallArguments;
use biome_formatter::FormatOptions;
use biome_formatter::VecBuffer;
use biome_formatter::format_element::LineMode;
use biome_formatter::format_element::tag::Tag;
use biome_formatter::write;
use biome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRArgument;
impl FormatNodeRule<RArgument> for FormatRArgument {
    fn fmt_fields(&self, node: &RArgument, f: &mut RFormatter) -> FormatResult<()> {
        fmt_argument_fields(node, f)
    }
}

pub(crate) fn fmt_argument_fields(node: &RArgument, f: &mut RFormatter) -> FormatResult<()> {
    let RArgumentFields { name_clause, value } = node.as_fields();

    match (name_clause, value) {
        // Hole
        // `foo(,)`
        // `foo(value, )`
        // `foo(, value)`
        (None, None) => Ok(()),

        // Unnamed argument
        // `foo(value)`
        // `foo(value, value)`
        (None, Some(value)) => write!(f, [value.format()]),

        // Named argument without a value
        // We write a mandatory space as a signal that this is a fairly
        // weird a nonstandard thing to see.
        // `foo(name = )`
        // `foo(name = , value)`
        (Some(name_clause), None) => write!(f, [name_clause.format(), space()]),

        // Named argument with a value
        // `foo(name = value)`
        (Some(name_clause), Some(value)) => {
            if is_in_test_call(node)
                || matches!(
                    value,
                    AnyRExpression::RBracedExpressions(_)
                        | AnyRExpression::RFunctionDefinition(_)
                        | AnyRExpression::RIfStatement(_)
                        | AnyRExpression::RForStatement(_)
                        | AnyRExpression::RWhileStatement(_)
                        | AnyRExpression::RRepeatStatement(_)
                )
                || is_call_with_long_callee(&value, f)
            {
                return write!(f, [name_clause.format(), space(), value.format()]);
            }

            let is_binary = matches!(value, AnyRExpression::RBinaryExpression(_));
            let format_value = format_with(|f| {
                let mut buffer = VecBuffer::new(f.state_mut());
                write!(buffer, [value.format()])?;
                let mut elements = buffer.into_vec();
                flatten_argument_value_soft_lines(&mut elements, is_binary, f);
                f.write_elements(elements)
            });

            let group_id = f.group_id("named_argument_value");
            write!(
                f,
                [
                    name_clause.format(),
                    group(&indent(&soft_line_break_or_space())).with_group_id(Some(group_id)),
                    indent_if_group_breaks(&format_value, group_id)
                ]
            )
        }
    }
}

/// Flattens soft line breaks in a named argument's value (or its `<left> <op>` prefix
/// for a multi-line binary expression) when that span has no forced breaks and fits on a
/// single indented line on its own, so `FitsMeasurer` measures the full single-line
/// expression (or `<left> <op>` prefix) rather than stopping at an earlier soft line break.
fn flatten_argument_value_soft_lines(
    elements: &mut Vec<FormatElement>,
    is_binary: bool,
    f: &RFormatter,
) {
    let max_flat_width = usize::from(f.options().line_width().value())
        .saturating_sub(usize::from(f.options().indent_width().value()) * 4);

    let prefix_len = if !elements.will_break() && flat_text_width(elements) <= max_flat_width {
        elements.len()
    } else if is_binary
        && let Some(idx) = first_binary_op_line_idx(elements)
        && flat_text_width(&elements[..idx]) <= max_flat_width
    {
        idx
    } else {
        return;
    };

    let mut remaining = prefix_len;
    elements.retain_mut(|item| {
        if remaining == 0 {
            return true;
        }
        remaining -= 1;
        if matches!(item, FormatElement::Line(LineMode::SoftOrSpace)) {
            *item = FormatElement::Space;
        }
        !matches!(item, FormatElement::Line(LineMode::Soft))
    });
}

fn first_binary_op_line_idx(slice: &[FormatElement]) -> Option<usize> {
    let mut group_depth = 0usize;
    let mut in_expanded_group = false;
    for (idx, item) in slice.iter().enumerate() {
        match item {
            FormatElement::Tag(Tag::StartGroup(group)) => {
                group_depth += 1;
                if group_depth > 1 && !group.mode().is_flat() {
                    in_expanded_group = true;
                }
            }
            FormatElement::Tag(Tag::EndGroup) => {
                group_depth = group_depth.saturating_sub(1);
            }
            FormatElement::Line(_) if group_depth == 1 || in_expanded_group => {
                return Some(idx);
            }
            other if other.will_break() => return None,
            _ => {}
        }
    }
    None
}

fn flat_text_width(slice: &[FormatElement]) -> usize {
    slice
        .iter()
        .map(|item| match item {
            FormatElement::StaticText { text } => text.len(),
            FormatElement::DynamicText { text, .. } => text.len(),
            FormatElement::LocatedTokenText { slice, .. } => usize::from(slice.len()),
            FormatElement::Space
            | FormatElement::HardSpace
            | FormatElement::Line(LineMode::SoftOrSpace) => 1,
            _ => 0,
        })
        .sum()
}

fn is_in_test_call(node: &RArgument) -> bool {
    node.parent::<RArgumentList>()
        .and_then(|list| list.parent::<RCallArguments>())
        .and_then(|args| args.parent::<RCall>())
        .is_some_and(|call| call.is_test_call().unwrap_or(false))
}

fn is_call_with_long_callee(value: &AnyRExpression, f: &RFormatter) -> bool {
    let callee = match value {
        AnyRExpression::RCall(call) => call.function(),
        AnyRExpression::RSubset(subset) => subset.function(),
        AnyRExpression::RSubset2(subset2) => subset2.function(),
        _ => return false,
    };
    callee.is_ok_and(|callee| {
        usize::from(callee.syntax().text_trimmed().len()) + 1
            > usize::from(f.options().line_width().value())
    })
}
