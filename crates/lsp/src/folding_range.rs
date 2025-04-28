//
// folding_range.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use regex::Regex;
use std::cmp::Ordering;
use std::sync::LazyLock;

use tower_lsp::lsp_types::FoldingRange;
use tower_lsp::lsp_types::FoldingRangeKind;

use crate::documents::Document;

pub fn folding_range(document: &Document) -> anyhow::Result<Vec<FoldingRange>> {
    let mut folding_ranges: Vec<FoldingRange> = Vec::new();

    // Activate the parser
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_r::LANGUAGE.into())
        .unwrap();

    let ast = parser.parse(&document.contents, None).unwrap();

    if ast.root_node().has_error() {
        tracing::error!("Folding range service: Parse error");
        return Err(anyhow::anyhow!("Parse error"));
    }

    // Traverse the AST
    let mut cursor = ast.root_node().walk();
    parse_ts_node(
        &mut cursor,
        0,
        &mut folding_ranges,
        document,
        &mut vec![Vec::new()],
        &mut None,
        &mut None,
    );

    // Add indent folding ranges
    append_indent_folding_ranges(document, &mut folding_ranges);
    Ok(folding_ranges)
}

fn parse_ts_node(
    cursor: &mut tree_sitter::TreeCursor,
    _depth: usize,
    folding_ranges: &mut Vec<FoldingRange>,
    document: &Document,
    comment_stack: &mut Vec<Vec<(usize, usize)>>,
    region_marker: &mut Option<usize>,
    cell_marker: &mut Option<usize>,
) {
    let node = cursor.node();
    let _field_name = match cursor.field_name() {
        Some(name) => format!("{name}: "),
        None => String::new(),
    };

    let start = node.start_position();
    let end = node.end_position();
    let node_type = node.kind();

    match node_type {
        "parameters" | "arguments" | "braced_expression" => {
            // ignore same line folding
            if start.row == end.row {
                return;
            }
            let folding_range = bracket_range(
                start.row,
                start.column,
                end.row,
                end.column - 1,
                count_leading_whitespaces(document, end.row),
            );
            folding_ranges.push(folding_range);
        }
        "comment" => {
            // Only process standalone comment
            if count_leading_whitespaces(document, start.row) != start.column {
                return;
            }

            // Nested comment section handling
            let comment_line = get_line_text(document, start.row, None, None);

            nested_processor(
                document,
                comment_stack,
                folding_ranges,
                start.row,
                &comment_line,
            );
            region_processor(folding_ranges, region_marker, start.row, &comment_line);
            cell_processor(folding_ranges, cell_marker, start.row, &comment_line);
        }
        _ => (),
    }

    if cursor.goto_first_child() {
        // create node child stacks
        // This is a stack of stacks for each bracket level, within each stack is a vector of (level, start_line) tuples
        let mut child_comment_stack: Vec<Vec<(usize, usize)>> = vec![Vec::new()];
        let mut child_region_marker: Option<usize> = None;
        let mut child_cell_marker: Option<usize> = None;

        // recursive loop
        loop {
            parse_ts_node(
                cursor,
                _depth + 1,
                folding_ranges,
                document,
                &mut child_comment_stack,
                &mut child_region_marker,
                &mut child_cell_marker,
            );
            if !cursor.goto_next_sibling() {
                break;
            }
        }
        // End of node handling
        end_node_handler(
            document,
            folding_ranges,
            end.row,
            &mut child_comment_stack,
            &mut child_region_marker,
            &mut child_cell_marker,
        );

        cursor.goto_parent();
    }
}

// Function to create a folding range that specifically deals with bracket ending
fn bracket_range(
    start_line: usize,
    start_char: usize,
    end_line: usize,
    end_char: usize,
    white_space_count: usize,
) -> FoldingRange {
    let mut end_line: u32 = end_line.try_into().unwrap();
    let mut end_char: Option<u32> = Some(end_char.try_into().unwrap());

    let adjusted_end_char = end_char.and_then(|val| val.checked_sub(white_space_count as u32));

    match adjusted_end_char {
        Some(0) => {
            end_line -= 1;
            end_char = None;
        }
        Some(_) => {
            if let Some(ref mut value) = end_char {
                *value -= 1;
            }
        }
        None => {
            tracing::error!(
                "Folding Range (bracket_range): adjusted_end_char should not be None here"
            );
        }
    }

    FoldingRange {
        start_line: start_line.try_into().unwrap(),
        start_character: Some(start_char as u32),
        end_line,
        end_character: end_char,
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: None,
    }
}

fn comment_range(start_line: usize, end_line: usize) -> FoldingRange {
    FoldingRange {
        start_line: start_line.try_into().unwrap(),
        start_character: None,
        end_line: end_line.try_into().unwrap(),
        end_character: None,
        kind: Some(FoldingRangeKind::Region),
        collapsed_text: None,
    }
}

fn get_line_text(
    document: &Document,
    line_num: usize,
    start_char: Option<usize>,
    end_char: Option<usize>,
) -> String {
    let text = &document.contents;
    // Split the text into lines
    let lines: Vec<&str> = text.lines().collect();

    // Ensure the start_line is within bounds
    if line_num >= lines.len() {
        return String::new(); // Return an empty string if out of bounds
    }

    // Get the line corresponding to start_line
    let line = lines[line_num];

    // Determine the start and end character indices
    let start_idx = start_char.unwrap_or(0); // Default to 0 if None
    let end_idx = end_char.unwrap_or(line.len()); // Default to the line's length if None

    // Ensure indices are within bounds for the line
    let start_idx = start_idx.min(line.len());
    let end_idx = end_idx.min(line.len());

    // Extract the substring and return it
    line[start_idx..end_idx].to_string() // TODO
}

fn find_last_non_empty_line(document: &Document, start_line: usize, end_line: usize) -> usize {
    for idx in (start_line..=end_line).rev() {
        if !get_line_text(document, idx, None, None).trim().is_empty() {
            return idx;
        }
    }
    start_line
}

fn count_leading_whitespaces(document: &Document, line_num: usize) -> usize {
    let line_text = get_line_text(document, line_num, None, None);
    line_text.chars().take_while(|c| c.is_whitespace()).count()
}

pub static RE_COMMENT_SECTION: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^\s*(#+)\s*(.*?)\s*[#=-]{4,}\s*$").unwrap());

fn parse_comment_as_section(comment: &str) -> Option<(usize, String)> {
    // Match lines starting with one or more '#' followed by some non-empty content and must end with 4 or more '-', '#', or `=`
    // Ensure that there's actual content between the start and the trailing symbols.
    if let Some(caps) = RE_COMMENT_SECTION.captures(comment) {
        let hashes = caps.get(1)?.as_str().len(); // Count the number of '#'
        let title = caps.get(2)?.as_str().trim().to_string(); // Extract the title text without trailing punctuations
        if title.is_empty() {
            return None; // Return None for lines with only hashtags
        }
        return Some((hashes, title)); // Return the level based on the number of '#' and the title
    }

    None
}

fn nested_processor(
    document: &Document,
    comment_stack: &mut Vec<Vec<(usize, usize)>>,
    folding_ranges: &mut Vec<FoldingRange>,
    line_num: usize,
    comment_line: &str,
) {
    let Some((level, _title)) = parse_comment_as_section(comment_line) else {
        return; // return if the line is not a comment section
    };
    if comment_stack.is_empty() {
        tracing::error!(
            "Folding Range: comment_stack should always contain at least one element here"
        );
        return;
    }
    loop {
        if comment_stack.last().unwrap().is_empty() {
            comment_stack.last_mut().unwrap().push((level, line_num));
            return; // return if the stack is empty
        }

        let Some((last_level, _)) = comment_stack.last().unwrap().last() else {
            tracing::error!("Folding Range: comment_stacks should not be empty here");
            return;
        };
        match last_level.cmp(&level) {
            Ordering::Less => {
                comment_stack.last_mut().unwrap().push((level, line_num));
                break;
            }
            Ordering::Equal => {
                let start_line = comment_stack.last().unwrap().last().unwrap().1;
                folding_ranges.push(comment_range(
                    start_line,
                    find_last_non_empty_line(document, start_line, line_num - 1),
                ));
                comment_stack.last_mut().unwrap().pop();
                comment_stack.last_mut().unwrap().push((level, line_num));
                break;
            }
            Ordering::Greater => {
                let start_line = comment_stack.last().unwrap().last().unwrap().1;
                folding_ranges.push(comment_range(
                    start_line,
                    find_last_non_empty_line(document, start_line, line_num - 1),
                ));
                comment_stack.last_mut().unwrap().pop(); // Safe: the loop exits early if the stack becomes empty
            }
        }
    }
}

fn region_processor(
    folding_ranges: &mut Vec<FoldingRange>,
    region_marker: &mut Option<usize>,
    line_idx: usize,
    line_text: &str,
) {
    let Some(region_type) = parse_region_type(line_text) else {
        return; // return if the line is not a region section
    };
    match region_type.as_str() {
        "start" => {
            region_marker.replace(line_idx);
        }
        "end" => {
            if let Some(region_start) = region_marker {
                let folding_range = comment_range(*region_start, line_idx);
                folding_ranges.push(folding_range);
                *region_marker = None;
            }
        }
        _ => {}
    }
}

fn parse_region_type(line_text: &str) -> Option<String> {
    // return the region type
    // "start": "^\\s*#\\s*region\\b"
    // "end": "^\\s*#\\s*endregion\\b"
    // None: otherwise
    let region_start = Regex::new(r"^\s*#\s*region\b").unwrap();
    let region_end = Regex::new(r"^\s*#\s*endregion\b").unwrap();

    if region_start.is_match(line_text) {
        Some("start".to_string())
    } else if region_end.is_match(line_text) {
        Some("end".to_string())
    } else {
        None
    }
}

fn cell_processor(
    // Almost identical to region_processor
    folding_ranges: &mut Vec<FoldingRange>,
    cell_marker: &mut Option<usize>,
    line_idx: usize,
    line_text: &str,
) {
    let cell_pattern: Regex = Regex::new(r"^#\s*(%%|\+)(.*)").unwrap();

    if !cell_pattern.is_match(line_text) {
    } else {
        let Some(start_line) = cell_marker else {
            cell_marker.replace(line_idx);
            return;
        };

        let folding_range = comment_range(*start_line, line_idx - 1);
        folding_ranges.push(folding_range);
        cell_marker.replace(line_idx);
    }
}

fn end_node_handler(
    document: &Document,
    folding_ranges: &mut Vec<FoldingRange>,
    line_idx: usize,
    comment_stack: &mut Vec<Vec<(usize, usize)>>,
    region_marker: &mut Option<usize>,
    cell_marker: &mut Option<usize>,
) {
    // Nested comment handling
    // Iterate over the last element of the comment stack and add it to the folding ranges by using the comment_range function
    if let Some(last_section) = comment_stack.last() {
        // Iterate over each (start level, start line) in the last section
        for &(_level, start_line) in last_section.iter() {
            // Add a new folding range for each range in the last section
            let folding_range = comment_range(
                start_line,
                find_last_non_empty_line(document, start_line, line_idx - 1),
            );

            folding_ranges.push(folding_range);
        }
    }
    // Remove the last element from the comment stack after processing
    comment_stack.pop();

    // Unclosed region handling
    if let Some(region_start) = region_marker {
        let folding_range = comment_range(*region_start, line_idx - 1);
        folding_ranges.push(folding_range);
        *region_marker = None;
    }

    // End cell Handling
    if let Some(cell_start) = cell_marker {
        let folding_range = comment_range(*cell_start, line_idx - 1);
        folding_ranges.push(folding_range);
        *cell_marker = None;
    }
}

fn append_indent_folding_ranges(document: &Document, folding_ranges: &mut Vec<FoldingRange>) {
    let lines: Vec<&str> = document.contents.lines().collect();
    // usize::MAX is used as a placeholder for start lines which should not be included in the folding range
    let mut indent_stack: Vec<(usize, usize)> = vec![(usize::MAX, 0)]; // (start_line, indent_level)
    let mut last_line_is_empty = true; // folding ranges should not start with empty lines

    for (line_idx, line) in lines.iter().enumerate() {
        let trimmed = line.trim_end();
        let indent = line.chars().take_while(|c| c.is_whitespace()).count();

        // Pop all deeper indents
        loop {
            let Some(&(start_line, start_indent)) = indent_stack.last() else {
                indent_stack = vec![(usize::MAX, indent)];
                break;
            };

            if trimmed.is_empty() {
                if last_line_is_empty {
                    break; // end of indent block handling has been done
                };
                end_indent_handler(folding_ranges, &mut indent_stack, line_idx - 1); // flush indent block
                break;
            }

            match start_indent.cmp(&indent) {
                Ordering::Less => {
                    if last_line_is_empty {
                        // we need to update the placeholder indent level
                        indent_stack = vec![(usize::MAX, indent)];
                        break;
                    }
                    indent_stack.push((line_idx - 1, indent));
                    break;
                }
                Ordering::Equal => break,
                Ordering::Greater => {
                    if start_line != usize::MAX {
                        folding_ranges.push(FoldingRange {
                            start_line: start_line as u32,
                            end_line: (line_idx - 1) as u32,
                            kind: Some(FoldingRangeKind::Region),
                            start_character: None,
                            end_character: None,
                            collapsed_text: None,
                        });
                    }
                    indent_stack.pop();
                }
            }
        }
        last_line_is_empty = trimmed.is_empty();
    }

    // Final flush: any unfinished indent block to End of Document
    let last_line = lines.len().saturating_sub(1);
    end_indent_handler(folding_ranges, &mut indent_stack, last_line);
}

// end of indent block handling
fn end_indent_handler(
    folding_ranges: &mut Vec<FoldingRange>,
    indent_stack: &mut Vec<(usize, usize)>,
    line_idx: usize,
) {
    for (start_line, _) in indent_stack.into_iter() {
        if *start_line == usize::MAX {
            continue; // Skip the placeholder
        }
        if line_idx > *start_line {
            folding_ranges.push(FoldingRange {
                start_line: *start_line as u32,
                end_line: line_idx as u32,
                kind: Some(FoldingRangeKind::Region),
                start_character: None,
                end_character: None,
                collapsed_text: None,
            });
        }
    }
    indent_stack.push((usize::MAX, 0)); // Add the placeholder back to the stack
}
