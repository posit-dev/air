# Changelog

# Development version

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
