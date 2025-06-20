//! The Rust representation of `air.toml`
//!
//! The names and types of the fields in this struct determine the names and types
//! that can be specified in the `air.toml`.
//!
//! Every field is optional at this point, nothing is "finalized".
//! Finalization is done in [TomlOptions::into_settings].
//!
//! Global options are specified at top level in the TOML file (though we don't have
//! any of those at the moment). All other options are nested within their own `[table]`.
//!
//! Note that the doc comments in this file directly influence `air.schema.json`, which
//! is generated with `just gen-schema`.

use std::path::Path;

use crate::settings::DefaultExcludePatterns;
use crate::settings::DefaultIncludePatterns;
use crate::settings::ExcludePatterns;
use crate::settings::FormatSettings;
use crate::settings::LineEnding;
use crate::settings::Settings;
use settings::IndentStyle;
use settings::IndentWidth;
use settings::LineWidth;
use settings::PersistentLineBreaks;
use settings::Skip;

/// Configuration for Air
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct TomlOptions {
    #[serde(flatten)]
    pub global: GlobalTomlOptions,
    pub format: Option<FormatTomlOptions>,
}

// NOTE: Just a placeholder for now, we don't currently have any global settings
/// Global options affecting multiple commands.
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct GlobalTomlOptions {}

/// Options to configure code formatting.
#[derive(Clone, Debug, PartialEq, Eq, Default, serde::Deserialize)]
#[serde(deny_unknown_fields, rename_all = "kebab-case")]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct FormatTomlOptions {
    /// # The line width at which the formatter prefers to wrap lines
    ///
    /// The value must be greater than or equal to `1` and less than or equal to `320`.
    ///
    /// While the formatter will attempt to format lines such that they remain within the
    /// `line-width`, it isn't a hard upper bound, and formatted lines may exceed the
    /// `line-width`.
    pub line_width: Option<LineWidth>,

    /// # The number of spaces per indentation level
    ///
    /// The value must be greater than or equal to `1` and less than or equal to `24`. The
    /// default value is `2`.
    ///
    /// Used by the formatter to determine the visual width of a tab.
    ///
    /// This option changes the number of spaces the formatter inserts when using
    /// `indent-style = "space"`. It also represents the width of a tab when `indent-style
    /// = "tab"` for the purposes of computing the `line-width`.
    pub indent_width: Option<IndentWidth>,

    /// # Whether to use spaces or tabs for indentation
    ///
    /// `indent-style = "space"` (default):
    ///
    /// ```r
    /// fn <- function() {
    ///   # Spaces indent `cat()`
    ///   cat("Hello")
    /// }
    /// ```
    ///
    /// `indent-style = "tab"`:
    ///
    /// ```r
    /// fn <- function() {
    ///   # A tab `\t` indents `cat()`
    ///   cat("Hello")
    /// }
    /// ```
    ///
    /// Air defaults to spaces due to the overwhelming amount of existing R code written
    /// in this style, but consider using tabs for new projects to improve accessibility.
    ///
    /// See `indent-width` to configure the number of spaces per indentation and the tab
    /// width.
    pub indent_style: Option<IndentStyle>,

    /// # The character used at the end of a line
    ///
    /// - `auto`: The newline style is detected automatically on a file per file basis.
    ///   Files with mixed line endings will be converted to the first detected line
    ///   ending. Defaults to `\n` for files that contain no line endings.
    ///
    /// - `lf`: Line endings will be converted to `\n`. The default line ending on Unix.
    ///
    /// - `crlf`: Line endings will be converted to `\r\n`. The default line ending on
    ///   Windows.
    ///
    /// - `native`: Line endings will be converted to `\n` on Unix and `\r\n` on Windows.
    pub line_ending: Option<LineEnding>,

    /// # Whether or not to respect persistent line breaks
    ///
    /// Air respects a small set of persistent line breaks as an indication that certain
    /// function calls or function signatures should be left expanded. If this option
    /// is set to `false`, persistent line breaks are ignored.
    ///
    /// It may be preferable to ignore persistent line breaks if you prefer that `line-width`
    /// should be the only value that influences line breaks.
    pub persistent_line_breaks: Option<bool>,

    /// # Patterns to exclude from formatting
    ///
    /// By default, Air will refuse to format files matched by patterns listed in
    /// `default-exclude`. Use this option to supply an additional list of exclude
    /// patterns.
    ///
    /// Exclude patterns are modeled after what you can provide in a
    /// [.gitignore](https://git-scm.com/docs/gitignore), and are resolved relative to the
    /// parent directory that your `air.toml` is contained within. For example, if your
    /// `air.toml` was located at `root/air.toml`, then:
    ///
    /// - `file.R` excludes a file named `file.R` located anywhere below `root/`. This is
    ///   equivalent to `**/file.R`.
    ///
    /// - `folder/` excludes a directory named `folder` (and all of its children) located
    ///   anywhere below `root/`. You can also just use `folder`, but this would
    ///   technically also match a file named `folder`, so the trailing slash is preferred
    ///   when targeting directories. This is equivalent to `**/folder/`.
    ///
    /// - `/file.R` excludes a file named `file.R` located at `root/file.R`.
    ///
    /// - `/folder/` excludes a directory named `folder` (and all of its children) located
    ///   at `root/folder/`.
    ///
    /// - `file-*.R` excludes R files named like `file-this.R` and `file-that.R` located
    ///   anywhere below `root/`.
    ///
    /// - `folder/*.R` excludes all R files located at `root/folder/`. Note that R files
    ///   in directories under `folder/` are not excluded in this case (such as
    ///   `root/folder/subfolder/file.R`).
    ///
    /// - `folder/**/*.R` excludes all R files located anywhere below `root/folder/`.
    ///
    /// - `**/folder/*.R` excludes all R files located directly inside a `folder/`
    ///   directory, where the `folder/` directory itself can /// appear anywhere.
    ///
    /// See the full [.gitignore](https://git-scm.com/docs/gitignore) documentation for
    /// all of the patterns you can provide.
    pub exclude: Option<Vec<String>>,

    /// # Whether or not to use default exclude patterns
    ///
    /// Air automatically excludes a default set of folders and files. If this option is
    /// set to `false`, these files will be formatted as well.
    ///
    /// The default set of excluded patterns are:
    /// - `.git/`
    /// - `renv/`
    /// - `revdep/`
    /// - `cpp11.R`
    /// - `RcppExports.R`
    /// - `extendr-wrappers.R`
    /// - `import-standalone-*.R`
    pub default_exclude: Option<bool>,

    /// # Function calls to skip formatting for
    ///
    /// Air typically formats every function call it comes across. To skip formatting of
    /// a single one-off function call, you can use a `# fmt: skip` comment. However, if
    /// you know of particular functions that you use a lot that are part of a custom
    /// domain specific language that doesn't follow conventional formatting rules, you
    /// can entirely opt out of formatting for those functions by providing them here.
    ///
    /// For example, using `skip = ["graph_from_literal"]` would automatically skip
    /// formatting of:
    ///
    /// ```r
    /// igraph::graph_from_literal(Alice +--+ Bob)
    /// ```
    ///
    /// In the short term, we also anticipate that this will be useful to avoid formatting
    /// of `tibble::tribble()` calls. In the long term, we may have more sophisticated
    /// features for automatically formatting using a specified alignment.
    pub skip: Option<Skip>,
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
            exclude: match format.exclude {
                Some(exclude) => {
                    let exclude = exclude.iter().map(String::as_str);
                    Some(ExcludePatterns::try_from_iter(root, exclude)?)
                }
                None => None,
            },
            default_exclude: match format.default_exclude.unwrap_or(true) {
                true => Some(DefaultExcludePatterns::default()),
                false => None,
            },
            // `include` and `default_include` are not currently exposed as toml options.
            // Theoretically could be for consistency, but there aren't any motivating use
            // cases right now.
            default_include: Some(DefaultIncludePatterns::default()),
            skip: format.skip,
        };

        Ok(Settings { format })
    }
}
