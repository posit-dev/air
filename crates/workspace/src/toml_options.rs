//
// toml_options.rs
//
// Copyright (C) 2025 Posit Software, PBC. All rights reserved.
//
//

use std::path::Path;

use crate::settings::FormatSettings;
use crate::settings::IgnorePatterns;
use crate::settings::IncludePatterns;
use crate::settings::LineEnding;
use crate::settings::Settings;
use settings::IndentStyle;
use settings::IndentWidth;
use settings::LineWidth;
use settings::PersistentLineBreaks;

/// The Rust representation of `air.toml`
///
/// The names and types of the fields in this struct determine the names and types
/// that can be specified in the `air.toml`.
///
/// Every field is optional at this point, nothing is "finalized".
/// Finalization is done in [TomlOptions::into_settings].
///
/// Global options are specified at top level in the TOML file.
/// All other options are nested within their own `[table]`.
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct TomlOptions {
    /// Global options affecting multiple commands.
    #[serde(flatten)]
    pub global: GlobalTomlOptions,

    /// Options to configure code formatting.
    pub format: Option<FormatTomlOptions>,
}

// NOTE: Just a placeholder for now, we don't currently have any global settings
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct GlobalTomlOptions {}

/// Configures the way air formats your code.
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct FormatTomlOptions {
    /// The line width at which the formatter prefers to wrap lines.
    ///
    /// The value must be greater than or equal to `1` and less than or equal to `320`.
    ///
    /// Note: While the formatter will attempt to format lines such that they remain
    /// within the `line-width`, it isn't a hard upper bound, and formatted lines may
    /// exceed the `line-width`.
    pub line_width: Option<LineWidth>,

    /// The number of spaces per indentation level (tab).
    ///
    /// The value must be greater than or equal to `1` and less than or equal to `24`. The
    /// default value is `2`.
    ///
    /// Used by the formatter to determine the visual width of a tab.
    ///
    /// This option changes the number of spaces the formatter inserts when
    /// using `indent-style = "space"`. It also represents the width of a tab when
    /// `indent-style = "tab"` for the purposes of computing the `line-width`.
    pub indent_width: Option<IndentWidth>,

    /// Whether to use spaces or tabs for indentation.
    ///
    /// `indent-style = "space"` (default):
    ///
    /// ```r
    /// fn <- function() {
    ///   cat("Hello") # Spaces indent the `cat()` call.
    /// }
    /// ```
    ///
    /// `indent-style = "tab"`:
    ///
    /// ```r
    /// fn <- function() {
    ///   cat("Hello") # A tab `\t` indents the `cat()` call.
    /// }
    /// ```
    ///
    /// Air defaults to spaces due to the overwhelming amount of existing R code written
    /// in this style, but consider using tabs for new projects to improve accessibility.
    ///
    /// See `indent-width` to configure the number of spaces per indentation and the tab width.
    pub indent_style: Option<IndentStyle>,

    /// The character air uses at the end of a line.
    ///
    /// - `auto`: The newline style is detected automatically on a file per file basis. Files with mixed line endings will be converted to the first detected line ending. Defaults to `\n` for files that contain no line endings.
    /// - `lf`: Line endings will be converted to `\n`. The default line ending on Unix.
    /// - `crlf`: Line endings will be converted to `\r\n`. The default line ending on Windows.
    /// - `native`: Line endings will be converted to `\n` on Unix and `\r\n` on Windows.
    pub line_ending: Option<LineEnding>,

    /// Air respects a small set of persistent line breaks as an indication that certain
    /// function calls or function signatures should be left expanded. If this option
    /// is set to `false`, persistent line breaks are ignored.
    ///
    /// It may be preferable to ignore persistent line breaks if you prefer that `line-width`
    /// should be the only value that influences line breaks.
    pub persistent_line_breaks: Option<bool>,

    /// By default, Air will refuse to format files listed in the set of `default_ignore`.
    /// To add to this list, use this option to supply a list of ignore patterns.
    ///
    /// Ignore patterns are modeled after what you can provide in a
    /// [.gitignore](https://git-scm.com/docs/gitignore), and are resolved relative to the
    /// parent directory that your `air.toml` is contained within. For example, if your
    /// `air.toml` was located at `root/air.toml`, then you could provide:
    ///
    /// - `ignore.R` to ignore that R file located anywhere below `root`.
    /// - `ignore/` to ignore that directory located anywhere below `root`. You can also
    ///   just use `ignore`, but this would technically also match a file named `ignore`,
    ///   so the trailing slash is preferred when targeting directories.
    /// - `/ignore/` to ignore a directory at `root/ignore/`, where the leading `/` forces
    ///   the directory to appear right under `root/`, rather than anywhere.
    /// - `ignore-*.R` to ignore R files like `ignore-this.R` and `ignore-that.R` located
    ///   anywhere below `root`.
    /// - `ignore/*.R` to ignore all R files at `root/ignore/*.R`, where the `/` in the
    ///   middle of the pattern forces the directory to appear right under `root/`, rather
    ///   than anywhere.
    /// - `**/ignore/*.R` to ignore all R files below an `ignore/` directory, where the
    ///   `ignore/` directory itself can appear anywhere.
    ///
    /// See the full [.gitignore](https://git-scm.com/docs/gitignore) documentation for
    /// all of the patterns you can provide.
    pub ignore: Option<Vec<String>>,

    /// Air automatically ignores a default set of folders and files. If this option is
    /// set to `false`, these files will be formatted as well.
    ///
    /// The default set of ignored patterns are:
    /// - `.git/`
    /// - `renv/`
    /// - `revdep/`
    /// - `cpp11.R`
    /// - `RcppExports.R`
    /// - `extendr-wrappers.R`
    /// - `import-standalone-*.R`
    pub default_ignore: Option<bool>,
}

impl TomlOptions {
    pub fn into_settings(self, root: &Path) -> anyhow::Result<Settings> {
        let format = self.format.unwrap_or_default();

        let format = FormatSettings {
            indent_style: format.indent_style.unwrap_or_default(),
            indent_width: format.indent_width.unwrap_or_default(),
            line_ending: format.line_ending.unwrap_or_default(),
            line_width: format.line_width.unwrap_or_default(),
            persistent_line_breaks: match format.persistent_line_breaks {
                Some(persistent_line_breaks) => {
                    if persistent_line_breaks {
                        PersistentLineBreaks::Respect
                    } else {
                        PersistentLineBreaks::Ignore
                    }
                }
                None => PersistentLineBreaks::Respect,
            },
            ignore: {
                let ignore = format.ignore.unwrap_or_default();
                let ignore = ignore.iter().map(String::as_str);
                let default_ignore = format.default_ignore.unwrap_or(true);
                IgnorePatterns::try_from_iter(root, ignore, default_ignore)?
            },
            // Not currently exposed as a toml option. Theoretically could be for
            // consistency, but there aren't any motivating use cases right now.
            include: IncludePatterns::default(),
        };

        Ok(Settings { format })
    }
}
