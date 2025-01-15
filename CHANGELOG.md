# Changelog

# Development version


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
