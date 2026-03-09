# Changelog

# Development version

- Air's behavior relating to directly supplied files and the `exclude` / `default-exclude` options has changed to be safer by default. Previously, `air format cpp11.R` would format `cpp11.R`, even though it was part of the set of `default-exclude`s, because we assumed that a direct request from a user like this could override these rules. However, tools such as pre-commit or IDEs that format via stdin will blindly call `air format` on any file that changes and have no knowledge of whether that file should be excluded or not. For this reason, we now exclude files that match `exclude` or `default-exclude` patterns even if they are directly supplied on the command line.

  Similarly, `air format my.qmd` no longer attempts to format `my.qmd`. This file is not excluded by `exclude` or `default-exclude`, but is also not included by our internal set of `default-include`s, which currently only accept `.R` and `.r` files. Rather than blindly trying to format this directly supplied file, Air now ignores it (#476).

- New `--force` flag to bypass all exclusion and inclusion rules and force a file or folder to be formatted. This flag applies recursively, meaning that all files within a forced folder will also be forcibly formatted, regardless of file type. This flag should rarely be needed, but serves as an escape hatch for cases like `air format r-code.txt --force`, which is no longer automatically formatted by Air as of this release due to the change mentioned above (#478).

- New `air generate-shell-completion <SHELL>` hidden command that emits a script to stdout that generates shell completions. Supports bash, zsh, fish, powershell, and elvish (#477, @salim-b).

  For zsh, run the following to add to your `.zshrc`:

  ```bash
  echo 'eval "$(air generate-shell-completion zsh)"' >> ~/.zshrc
  ```

  For powershell, run the following to add to your profile:

  ```powershell
  if (!(Test-Path -Path $PROFILE)) {
    New-Item -ItemType File -Path $PROFILE -Force
  }
  Add-Content -Path $PROFILE -Value '(& air generate-shell-completion powershell) | Out-String | Invoke-Expression'
  ```

  Then restart your shell and type `air <tab>` to see completions.

# 0.8.2

- Air is now distributed on PyPI as `air-formatter` (#467).

  This allows air to be invoked via uv:

  ```bash
  # Global install of `air`
  uv tool install air-formatter
  air format .

  # One-off run
  uvx --from air-formatter air format .
  ```

- Air is now code-signed on Windows (#461).


# 0.8.1

- Added color to Air's terminal help. Disable by setting the environment variable `NO_COLOR=1` (#447, @etiennebacher).

- Fixed an issue with Unicode elements and table alignment (#449).


# 0.8.0

- Added support for table formatting of `tribble()` and `fcase()` calls (#113).
  You can also opt into table formatting for any other call with the `# fmt: table` comment directive, or the `table` TOML option. See also the `default-table` option to turn off Air's defaults for `tribble()` and `fcase()`.

  Note: This feature is experimental. We'd be grateful for any feedback!

- Formulas are now treated like assignment operators rather than like comparison operators, which means they now left-align expression chains on the right-hand side of the formula, respect persistent line breaks, and never automatically break around the `~` operator itself (#336, #402).

  With model formulas:

  ```r
  # Before:
  y ~
    year +
      age +
      size

  # After:
  y ~
    year +
    age +
    size
  ```

  With complex `case_when()` calls:

  ```r
  # Before:
  case_when(
    x %in% c(1, 2) ~
      {
        this + complex + thing
      },
    x %in% c(3, 4) ~
      {
        that + thing
      }
  )

  # After:
  case_when(
    x %in% c(1, 2) ~ {
      this + complex + thing
    },
    x %in% c(3, 4) ~ {
      that + thing
    }
  )
  ```


# 0.7.1

- We now recommend using [Tombi](https://github.com/tombi-toml/tombi) for `air.toml` autocompletion and validation instead of Even Better TOML. Tombi is easily installable from the [VS Code Marketplace](https://marketplace.visualstudio.com/items?itemName=tombi-toml.tombi), the [OpenVSX Marketplace](https://open-vsx.org/namespace/tombi-toml), as a [Zed extension](https://zed.dev/extensions?query=tombi&filter=language-servers), or using some other [supported installation method](https://tombi-toml.github.io/tombi/docs/installation). We've improved on our `air.toml` configuration documentation to help tombi provide the best `air.toml` editing experience (#371).


# 0.7.0

- Autobracing is a new feature applied to if statements, for loops, while loops, repeat loops, and function definitions. This feature will automatically add `{}` around the body of these code elements in certain cases to maximize readability, consistency, and portability (#225, #334).

  For example:

  ```r
  if (condition)
    a

  # Becomes:
  if (condition) {
    a
  }
  ```

  ```r
  fn <- function(
    a, b
  ) a + b

  # Becomes:
  fn <- function(
    a,
    b
  ) {
    a + b
  }
  ```

  Single line if statements and function definitions are still allowed in certain contexts:

  ```r
  list(a = if (is.null(x)) NA else x)

  map(xs, function(x) x + 1)
  ```

- Empty `{}` are no longer ever expanded (#43).

  This allows for syntax like:

  ```r
  dummy <- function() {}

  while (waiting()) {}

  switch(x, a = {}, b = 2)

  function(
    expr = {}
  ) {
    this_first()
    expr
  }
  ```

- Binary exponents are now supported in hexadecimal constants (#357).

- `NULL` is now allowed in function call argument name position (#357).

- Fixed a case where some valid raw strings would cause a parse error (#255).


# 0.6.0

- Added documentation on using Air's GitHub Action, [setup-air](https://github.com/posit-dev/setup-air).

- Added documentation on using Air in [Zed](https://github.com/zed-industries/zed).


# 0.5.0

- Added support for a `skip` field in `air.toml` (#273).

  This is an extension of the `# fmt: skip` comment feature that provides a single place for you to list functions you never want formatting for. For example:

  ```toml
  skip = ["tribble", "graph_from_literal"]
  ```

  This `skip` configuration would skip formatting for these function calls, even without a `# fmt: skip` comment:

  ```r
  tribble(
    ~x, ~y,
     1,  2,
     3,  4
  )

  igraph::graph_from_literal(A +-+ B +---+ C ++ D + E)
  ```

  We expect this to be useful when working with packages that provide domain specific languages that come with their own unique formatting conventions.

- Fixed an issue where `air.toml` settings were not being applied to the correct R files (#294).


# 0.4.1

- Language server configuration variables are now fully optional, avoiding issues in editors like Zed or Helix (#246).


# 0.4.0

- Parenthesized expressions now tightly hug (#248).

- We now allow up to 2 lines between top-level elements of a file. This makes it possible to separate long scripts into visually distinct sections (#40).

- Unary formulas (i.e. anonymous functions) like `~ .x + 1` now add a space between the `~` and the right-hand side, unless the right-hand side is very simple, like `~foo` or `~1` (#235).

- Semicolons at the very start or very end of a file no longer cause the parser to panic (#238).

- Assigned pipelines no longer double-indent when a persistent line break is used (#220).

- Hugging calls like:

  ```r
  list(c(
    1,
    2
  ))
  ```

  are no longer fully expanded (#21).

- Assigned pipelines no longer double-indent (#220).

- Added support for special "skip" comments.

  Use `# fmt: skip` to avoid formatting the following node and all of its children. In this case, the `tribble()` call and all of its arguments (#52).

  ```r
  # fmt: skip
  tribble(
    ~a, ~b,
     1,  2
  )
  ```

  Use `# fmt: skip file` to avoid formatting an entire file. This comment must appear at the top of the file before any non-comment R code (#219).


# 0.3.0

- Air has gained support for excluding files and folders (#128).

  - Air now excludes a set of default R files and folders by default. These
    include generated files such as `cpp11.R` and `RcppExports.R`, as well as
    folders that may contain such files, like `renv/` and `revdep/`. If you'd
    prefer to have Air format these files as well, set the new
    `default-exclude` option to `false`.

  - To add additional files or folders to exclude, use the new `exclude` option.
    This accepts a list of `.gitignore` style patterns, such as
    `exclude = ["file.R", "folder/", "files-like-*-this.R"]`.

- Linux binaries are now available. Note that your Linux distribution must
  support glibc 2.31+ for the binary to work (#71).

- ARM Windows binaries are now available (#170).


# 0.2.0

- Initial public release, yay!

  Note that we first released 0.2.0 as 1.0.0. If you have installed the VS Code extension or the CLI program as 1.0.0, please uninstall it.

- Fixed an issue where the language server failed to start due to logging
  being initialized twice.

- Added a synchronization mechanism between IDE and Air settings. See documentation for more information https://posit-dev.github.io/air/configuration.html#settings-synchronization.

- Renamed `ignore-magic-line-break` to `persistent-line-breaks` (#177).

- In the CLI, errors and warnings are now written to stderr. This allows you to
  see issues that occur during `air format`, such as parse errors or file not
  found errors (#155).

- New global CLI option `--log-level` to control the log level. The default is
  `warn` (#155).

- New global CLI option `--no-color` to disable colored output (#155).

- Air now supports `.air.toml` files in addition to `air.toml` files. If both
  are in the same directory, `air.toml` is preferred, but we don't recommend
  doing that (#152).


# 0.1.2

- The default indent style has been changed to spaces. The default indent width
  has been changed to two. This more closely matches the overwhelming majority
  of existing R code.

- Parse errors in your document no longer trigger an LSP error when you request
  document or range formatting (which typically would show up as an annoying
  toast notification in your code editor) (#120).

- `air format` is now faster on Windows when nothing changes (#90).

- `air format --check` now works correctly with Windows line endings (#123).

- Magic line breaks are now supported in left assignment (#118).


# 0.1.1

- The LSP gains range formatting support (#63).

- The `air format` command has been improved and is now able to take multiple files and directories.


# 0.1.0

- Initial release.
