use crate::prelude::*;
use crate::r::auxiliary::argument::fmt_argument_fields;
use crate::r::auxiliary::call_arguments::FormatRCallArguments;
use air_r_syntax::{
    AnyRExpression, AnyRValue, RArgument, RArgumentList, RCallArguments, RDoubleValue,
    RIntegerValue, RSyntaxToken, RUnaryExpression,
};

use biome_formatter::{CstFormatContext, FormatElement, RemoveSoftLinesBuffer, format_args, write};
use biome_rowan::AstSeparatedList;

const DOT_WIDTH: usize = 1;

#[derive(Debug, Clone)]
struct ArgData {
    node: RArgument,
    parts: ArgParts,
    separator: Option<RSyntaxToken>,
    is_last_in_list: bool,
}

#[derive(Debug, Clone)]
enum ArgParts {
    Numeric {
        integer_len: usize,
        fractional_len: Option<usize>,
    },
    Other {
        text: String,
    },
}

#[derive(Debug, Default, Clone)]
struct ColumnInfo {
    // It's possible to have `has_decimal = true` and `max_fractional_part = 0`
    // at the same time, if there is an argument like `1.`
    has_decimal: bool,
    max_width: usize,
    max_integer_part: usize,
    max_fractional_part: usize,
}

impl FormatRCallArguments {
    pub(crate) fn fmt_tabular(
        &self,
        node: &RCallArguments,
        f: &mut RFormatter,
    ) -> FormatResult<Option<()>> {
        let l_token = node.l_paren_token()?;
        let r_token = node.r_paren_token()?;
        let args = node.items();

        if args.is_empty() {
            return self.fmt_call_like(node, f).map(Some);
        }

        // Get table and alignment info
        let (rows, column_info) = match build_table(&args, f)? {
            Some(result) => result,
            None => return Ok(None),
        };

        // Format with alignment
        let formatted_table = format_with(|f| {
            for (row_i, row) in rows.iter().enumerate() {
                if row_i > 0 {
                    // Rows are separated with hard line breaks because the
                    // lines of arguments in tabular calls should never be
                    // rearranged by the formatter.
                    write!(f, [hard_line_break()])?;
                }

                for (col_j, arg_data) in row.iter().enumerate() {
                    let (left_pad, right_pad) = if column_info[col_j].max_width > 0 {
                        column_info[col_j].padding(&arg_data.parts)
                    } else {
                        // Empty columns are not padded, so that commas stick to
                        // each other as in regular calls: `list(,,,)`
                        (0, 0)
                    };

                    // Apply left padding with incompressible whitespace
                    if left_pad > 0 {
                        write!(f, [dynamic_text(&" ".repeat(left_pad), 0.into())])?;
                    }

                    // Write the argument
                    match &arg_data.parts {
                        ArgParts::Other { text } => {
                            let arg_syntax = arg_data.node.syntax();

                            // We've formatted the argument without comments, so
                            // we're in charge of formatting them
                            format_leading_comments(arg_syntax).fmt(f)?;

                            // 0-length arguments are holes. Don't print them
                            // because a `text("")` after a `Space` will prevent
                            // the latter from being considered trailing by the
                            // printer, and won't be removed if trailing.
                            if text.len() > 0 {
                                write!(f, [dynamic_text(text, 0.into())])?;
                            }

                            format_trailing_comments(arg_data.node.syntax()).fmt(f)?;
                        }

                        _ => {
                            // For numeric types, format the node directly. This
                            // handles comments as well.
                            write!(f, [arg_data.node.format()])?;
                        }
                    }

                    // These are hard spaces rather than `space()` to prevent
                    // the printer from compressing them down to one space.
                    // For this reason we need to be careful not to insert any
                    // space if the argument is the very last. Note that if
                    // there is a trailing comma, the last argument is
                    // technically the hole, so we still get padding for the
                    // last logical argument.
                    if !arg_data.is_last_in_list && right_pad > 0 {
                        write!(f, [dynamic_text(&" ".repeat(right_pad), 0.into())])?;
                    }

                    // Handle separators
                    if let Some(sep) = &arg_data.separator {
                        write!(f, [sep.format()])?;
                    } else if !arg_data.is_last_in_list {
                        // Shouldn't happen: A non-trailing argument without a separator
                        return Err(FormatError::SyntaxError);
                    }

                    // Only add space if the next column has non-empty elements,
                    // so that commas stick together as in `list(,,)`
                    if let Some(next_col) = column_info.get(col_j + 1) {
                        if next_col.max_width > 0 {
                            // This must be a soft space so it can be removed by
                            // the printer if trailing
                            write!(f, [space()])?;
                        }
                    }
                }
            }

            Ok(())
        });

        write!(
            f,
            [group(&format_args![
                l_token.format(),
                soft_block_indent(&formatted_table),
                r_token.format()
            ])
            .should_expand(true)]
        )
        .map(Some)
    }
}

// First pass: Parse arguments into rows and gather column width information for
// alignment. This involves finding the integer and fractional widths of numeric
// arguments, and formatting other arguments in a flat layout to get the width of
// the final printed text.
fn build_table(
    args: &RArgumentList,
    f: &mut RFormatter,
) -> FormatResult<Option<(Vec<Vec<ArgData>>, Vec<ColumnInfo>)>> {
    let mut rows: Vec<Vec<ArgData>> = Vec::new();
    let mut current_row = Vec::new();
    let mut column_info: Vec<ColumnInfo> = Vec::new();

    for (i, arg) in args.elements().enumerate() {
        let arg_node = arg.node()?;
        let arg_separator = arg.trailing_separator()?;

        let lines_before = if arg_node.value().is_some() {
            get_lines_before(arg_node.syntax())
        } else {
            arg_separator.map_or(0, get_lines_before_token)
        };

        if lines_before > 0 && !current_row.is_empty() {
            rows.push(current_row);
            current_row = Vec::new();
        }

        let column_index = current_row.len();
        while column_info.len() <= column_index {
            column_info.push(ColumnInfo::default());
        }

        let arg_parts = match ArgParts::parse(arg_node, f)? {
            Some(parts) => parts,
            None => return Ok(None),
        };

        let width = arg_parts.width();
        let col = &mut column_info[column_index];
        col.max_width = col.max_width.max(width);

        if let ArgParts::Numeric {
            integer_len,
            fractional_len,
        } = arg_parts
        {
            col.max_integer_part = col.max_integer_part.max(integer_len);
            if let Some(frac_len) = fractional_len {
                // Mark that this column contains decimals so padding logic can
                // align at the decimal point.
                col.has_decimal = true;
                col.max_fractional_part = col.max_fractional_part.max(frac_len);
            }
        }

        current_row.push(ArgData {
            node: arg_node.clone(),
            parts: arg_parts,
            separator: arg_separator.cloned(),
            is_last_in_list: i == args.len() - 1,
        });
    }

    if !current_row.is_empty() {
        rows.push(current_row);
    }

    Ok(Some((rows, column_info)))
}

impl ColumnInfo {
    fn padding(&self, arg: &ArgParts) -> (usize, usize) {
        if self.has_decimal {
            // We need to pad all arguments in decimal columns to the widest
            // element (either a numeric value or an "Other" argument) so that
            // commas align vertically, even if some arguments are wider than
            // the decimal alignment width.

            // This is the width needed to align all numeric arguments at the
            // decimal point
            let max_decimal_width = self.max_integer_part + DOT_WIDTH + self.max_fractional_part;

            // This is the width needed to ensure that even the widest "Other"
            // argument doesn't push the comma out of alignment
            let target_width = self.max_width.max(max_decimal_width);

            match arg {
                ArgParts::Numeric {
                    integer_len,
                    fractional_len,
                } => {
                    // Align at decimal point
                    let left = self.max_integer_part.saturating_sub(*integer_len);
                    let base_right = match fractional_len {
                        Some(frac) => self.max_fractional_part.saturating_sub(*frac),
                        None => DOT_WIDTH + self.max_fractional_part,
                    };

                    // Add extra padding if `max_width` (from an Other argument)
                    // exceeds `decimal_width`
                    let extra_right = target_width.saturating_sub(max_decimal_width);

                    (left, base_right + extra_right)
                }
                ArgParts::Other { .. } => {
                    // Left-align with right padding to reach target width
                    (0, target_width.saturating_sub(arg.width()))
                }
            }
        } else {
            // Non-decimal columns: All arguments are padded to `max_width`
            let padding = self.max_width.saturating_sub(arg.width());

            match arg {
                ArgParts::Numeric { .. } => (padding, 0), // Right-align
                ArgParts::Other { .. } => (0, padding),   // Left-align
            }
        }
    }
}

impl ArgParts {
    fn parse(arg: &RArgument, f: &mut RFormatter) -> FormatResult<Option<Self>> {
        match arg.value() {
            Some(AnyRExpression::AnyRValue(AnyRValue::RIntegerValue(value))) => {
                Ok(Some(Self::parse_integer(value)?))
            }
            Some(AnyRExpression::AnyRValue(AnyRValue::RDoubleValue(value))) => {
                Ok(Some(Self::parse_decimal(value)?))
            }
            Some(AnyRExpression::RUnaryExpression(value)) => Self::parse_unary(arg, value, f),
            Some(AnyRExpression::AnyRValue(AnyRValue::RBogusValue(_))) => {
                Err(FormatError::SyntaxError)
            }
            _ => Self::parse_other(arg, f),
        }
    }

    // Delegate to numerical parsing, but add 1 to the integer part for the
    // unary operator. Note that repeated unary operators like `--1` wil fall
    // back to ordinary parsing.
    fn parse_unary(
        arg: &RArgument,
        unary: RUnaryExpression,
        f: &mut RFormatter,
    ) -> FormatResult<Option<Self>> {
        let operator = unary.operator()?;
        let argument = unary.argument()?;

        let ("+" | "-") = operator.text_trimmed() else {
            return Self::parse_other(arg, f);
        };

        let parts = match argument {
            AnyRExpression::AnyRValue(AnyRValue::RIntegerValue(value)) => {
                Self::parse_integer(value)?
            }
            AnyRExpression::AnyRValue(AnyRValue::RDoubleValue(value)) => {
                Self::parse_decimal(value)?
            }
            _ => return Self::parse_other(arg, f),
        };

        let ArgParts::Numeric {
            integer_len,
            fractional_len,
        } = parts
        else {
            unreachable!();
        };

        return Ok(Some(ArgParts::Numeric {
            integer_len: integer_len + 1,
            fractional_len,
        }));
    }

    fn parse_integer(value: RIntegerValue) -> FormatResult<Self> {
        let token = value.value_token()?;

        // At least two characters, e.g. `0L`. The token length includes the suffix.
        Ok(ArgParts::Numeric {
            integer_len: token.text_trimmed().len(),
            fractional_len: None,
        })
    }

    fn parse_decimal(value: RDoubleValue) -> FormatResult<Self> {
        let token = value.value_token()?;
        let text = token.text_trimmed();

        if let Some(dot_pos) = text.find('.') {
            let integer_len = dot_pos;

            // Note this might be 0, e.g. `1.`
            let fractional_len = text[dot_pos + DOT_WIDTH..].len();

            Ok(ArgParts::Numeric {
                integer_len,
                fractional_len: Some(fractional_len),
            })
        } else {
            Ok(ArgParts::Numeric {
                integer_len: text.len(),
                fractional_len: None,
            })
        }
    }

    fn parse_other(arg: &RArgument, f: &mut RFormatter) -> FormatResult<Option<Self>> {
        let comments = f.context().comments();
        comments.mark_suppression_checked(arg.syntax());

        let snapshot = f.snapshot();

        let result = (|| {
            let mut buffer = RemoveSoftLinesBuffer::new(f);
            let mut recording = buffer.start_recording();
            write!(recording, [format_with(|f| fmt_argument_fields(arg, f))])?;
            let recorded = recording.stop();

            let ir: Vec<FormatElement> = recorded.into_iter().cloned().collect();
            let document = Document::from(ir);

            let formatted = biome_formatter::Formatted::new(document, f.context().clone());
            let text = formatted.print()?.into_code();

            // `will_break()` should not fail on us since we're formatting with
            // soft breaks diabled, but detecting newlines in the printed output
            // is the most reliable approach. Since we already need the text to
            // compute the argument width, we might as well do that.
            if text.contains("\n") {
                return Ok(None);
            }

            Ok(Some(ArgParts::Other { text }))
        })();

        f.restore_snapshot(snapshot);
        result
    }

    fn width(&self) -> usize {
        match self {
            ArgParts::Numeric {
                integer_len,
                fractional_len,
            } => match fractional_len {
                Some(frac_len) => *integer_len + DOT_WIDTH + *frac_len,
                None => *integer_len,
            },
            ArgParts::Other { text } => text.len(),
        }
    }
}
