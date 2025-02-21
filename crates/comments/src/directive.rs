#[derive(Debug, PartialEq)]
pub enum Directive {
    Format(FormatDirective),
}

#[derive(Debug, PartialEq)]
pub enum FormatDirective {
    Skip,
    SkipFile,
}

/// Parse a comment directive
///
/// These take the form:
///
/// ```text
/// # <category>: <command> <optional-argument>
/// ```
///
/// Such as:
///
/// ```text
/// # fmt: skip
/// # fmt: skip file
/// # fmt: tabular
/// # fmt: align-right
/// # lint: skip
/// # lint: skip rule
/// ```
///
/// Note that directives are applied to the node they are attached to.
pub fn parse_comment_directive(text: &str) -> Option<Directive> {
    let text = text.strip_prefix('#')?;
    let text = text.trim_start();
    let (category, text) = text.split_once(':')?;
    let text = text.trim();

    match category {
        "fmt" => parse_format_directive(text),
        _ => None,
    }
}

#[inline]
fn parse_format_directive(text: &str) -> Option<Directive> {
    match text {
        "skip" => Some(Directive::Format(FormatDirective::Skip)),
        "skip file" => Some(Directive::Format(FormatDirective::SkipFile)),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use crate::parse_comment_directive;
    use crate::Directive;

    #[test]
    fn test_format_directive() {
        let format_skip = Some(Directive::Format(crate::FormatDirective::Skip));
        let format_skip_file = Some(Directive::Format(crate::FormatDirective::SkipFile));

        // Must have leading `#`
        assert!(parse_comment_directive("fmt: skip").is_none());

        // Must have `:`
        assert!(parse_comment_directive("# fmt skip").is_none());

        // `:` must be right after `fmt`
        assert!(parse_comment_directive("# fmt : skip").is_none());

        // Can't have extra spaces between `skip file`
        assert!(parse_comment_directive("# fmt: skip  file").is_none());

        // Can't have unrelated leading text
        assert!(parse_comment_directive("# please fmt: skip").is_none());

        // Can't have unrelated trailing text
        assert!(parse_comment_directive("# fmt: skip please").is_none());
        assert!(parse_comment_directive("# fmt: skip file please").is_none());

        assert_eq!(parse_comment_directive("# fmt: skip"), format_skip);
        assert_eq!(parse_comment_directive("#fmt:skip"), format_skip);
        assert_eq!(parse_comment_directive("#  fmt:  skip  "), format_skip);

        assert_eq!(
            parse_comment_directive("# fmt: skip file"),
            format_skip_file
        );
        assert_eq!(parse_comment_directive("#fmt:skip file"), format_skip_file);
        assert_eq!(
            parse_comment_directive("#  fmt:  skip file"),
            format_skip_file
        );
    }
}
