use biome_text_size::{TextRange, TextSize};

pub(crate) fn extract_marked_range(input: &str) -> (String, TextRange) {
    let mut output = String::new();
    let mut start = None;
    let mut end = None;
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '<' && chars.peek() == Some(&'<') {
            chars.next();
            start = Some(TextSize::from(output.len() as u32));
        } else if c == '>' && chars.peek() == Some(&'>') {
            chars.next();
            end = Some(TextSize::from(output.len() as u32));
        } else {
            output.push(c);
        }
    }

    let range = match (start, end) {
        (Some(start), Some(end)) => TextRange::new(start, end),
        _ => panic!("Missing range markers"),
    };

    (output, range)
}
