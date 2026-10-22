# Parse diagnostics in the CLI

Status: draft, ready for review.

## Goal

When `air format` can't parse a file, show the user where and why, with a code
frame, instead of a single opaque line.

Today:

```
ERROR Failed to format R/foo.R: Failed to parse due to syntax errors.
```

After:

```
R/foo.R:3:8 parse ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  × Expected an expression but instead found ')'.

    1 │ f <- function(x) {
    2 │   y <- 1
  > 3 │   g(x, )
      │        ^
    4 │ }
    5 │

ERROR Failed to parse 1 file.
```

This resolves the `TODO(aether)` in `crates/workspace/src/format.rs`.

## Where things stand

- `air_r_parser::parse()` returns an `RParse` whose `diagnostics()` are
  `biome_parser::diagnostic::ParseDiagnostic`s. Each carries a `TextRange`
  span, a message, and optional detail advices with their own spans, as in
  "Reached the end of the file looking for a closing delimiter".

- `workspace::format::format_source()` already hands them back, in
  `FormatSourceError::Parse(Vec<ParseDiagnostic>)`. The CLI only ever prints
  that variant's `Display`, so the diagnostics are dropped.

- `format_path()` in `crates/air/src/commands/format/paths.rs` and
  `format_stdin()` in `stdin.rs` both drop the source text on the error
  path. The renderer needs that text to turn byte offsets into lines and
  columns.

- `air_r_parser_tests` already renders these diagnostics into the corpus
  snapshots, using `biome_diagnostics::PrintDiagnostic::verbose()` with
  `with_file_path()` and `with_file_source_code()`. The CLI takes the same
  path. Both move to `simple()`, which prints the same bytes as `verbose()`
  for parse diagnostics today, so users see what the corpus snapshots show.

- How air decides on color today is inconsistent:

  - `colored` checks whether stdout is a terminal, but everything it colors
    goes to stderr.

  - `tracing-subscriber` only checks `NO_COLOR`, and never checks for a
    terminal.

  So `air format . 2>log` run in a terminal writes ANSI codes into `log`.

- The LSP never calls `format_source()`. It checks `has_errors()` itself and
  is unaffected.

## Renderer

Use `biome_diagnostics`, which is already a workspace dependency. Things
worth knowing about it:

- Columns are counted in characters, not bytes, so non-ASCII R code reports
  the right column. Lines split on `\n`, `\r\n`, and a lone `\r`, and the
  frame trims line endings, so CRLF files render cleanly. This matters
  because the CLI doesn't normalize line endings.

- Frames show two lines of context, and frames longer than 8 lines get an
  ellipsis.

- Tabs are printed as tabs, and carets are padded by display width, so
  markers line up under tabs and wide characters.

- The `━` rule after the header is padded to 100 columns in debug builds and
  to the terminal width in release builds. It measures the terminal on
  stdout, with a fallback of 100. Integration tests run a debug binary, so
  snapshots are stable.

- The header changes with the terminal: under `TERM_PROGRAM=vscode` it skips
  the hyperlink, under JetBrains it prefixes ` at `, and otherwise an
  absolute path becomes an OSC 8 `file://` link. We pass relative paths
  whenever the file is under the working directory, so in practice only
  files outside it get links. This is acceptable as is.

- On Windows, `biome_console` swaps a few emoji for ASCII. `×` is already
  ASCII-safe, and the corpus snapshots pass on Windows CI today.

## Commits

1. Decide color once, based on stderr. This fixes the existing inconsistency
   on its own, before any diagnostics work.

2. Render parse diagnostics.

## Design

### Output streams

stdout carries only formatted code, in stdin mode. Everything else goes to
stderr, as it does today. That includes the code frames, the `ERROR` lines,
the summary, and `Would reformat:`.

For comparison:

- biome sends diagnostics to stderr, and the run summary to stdout.

- `ruff check` and `ty check` send diagnostics to stdout, as the command's
  result.

- `ruff format` puts parse errors on stdout as code frames under `--check`.
  In write mode and stdin mode, it falls back to one-line `error!` messages on
  stderr, because stdout is taken.

Air can't use stdout for diagnostics, because in stdin mode stdout is the
formatted file. Using stderr everywhere gives full code frames in every mode,
including the editor's stdin path. Moving `Would reformat:` to stdout to match
ruff would be a breaking change, and is out of scope.

### Color

Commit 1, in `crates/air/src/logging.rs`, or wherever `run()` sets up
logging:

- Compute one `bool` at startup:
  `!no_color && NO_COLOR is unset or empty && stderr().is_terminal()`.

- Apply it everywhere, with `colored::control::set_override(color)` and
  `layer.with_ansi(color)`. This replaces `turn_off_colors()`.
  `set_override(true)` is only reached once `NO_COLOR` has already been
  checked, so it never overrides it.

- Every later caller reads `colored::control::SHOULD_COLORIZE.should_colorize()`,
  as ruff does, so there is one source of truth and nothing to pass around.

- `CLICOLOR` and `CLICOLOR_FORCE` stop having an effect. `colored` only
  respected them by accident, and they are not documented in `--help`.
  `FORCE_COLOR` is not added.

Commit 2 maps `should_colorize()` to `ColorChoice::Always` or
`ColorChoice::Never` for the `termcolor::StandardStream::stderr()` the
diagnostics are written to.

### `workspace`

- Delete the `TODO(aether)` comment. `FormatSourceError::Parse` keeps its
  payload and its `Display`, which stays the fallback for any caller that
  doesn't render diagnostics.

### `air` crate

- Add `biome_console` and `biome_diagnostics` as dependencies.
  `biome_diagnostics` re-exports `termcolor`, so nothing else is needed.

- New module `crates/air/src/commands/format/diagnostics.rs`, shared by
  `paths.rs` and `stdin.rs`, with two functions:

  - A `ParseFailure` struct, holding a display path, the source text, and
    the `Vec<ParseDiagnostic>`.

  - `print_parse_failures(failures: &[ParseFailure], writer: &mut impl WriteColor)`,
    which renders every diagnostic of every failure, in order.

- Switch `render_diagnostics()` in `crates/air_r_parser_tests/src/check.rs`
  from `verbose()` to `simple()`, so the corpus snapshots keep showing exactly
  what the CLI prints. `ParseDiagnostic` has no verbose advices, so no
  snapshot changes, and `just accept` should produce no diff.

- Each diagnostic is rendered as biome's terminal reporter does:
  `diagnostic.with_file_path(path).with_file_source_code(source)`, then
  `PrintDiagnostic::simple()` through
  `biome_console::fmt::Formatter::new(&mut Termcolor(writer))`. Attaching the
  source copies it once per diagnostic. That's fine, since this only runs
  when parsing fails.

- Print every diagnostic. There is no cap and no `--max-diagnostics` flag.

### `paths.rs`

- `FormatPathError` gains a `Parse(ParseFailure)` variant. `format_path()`
  matches `FormatSourceError::Parse(diagnostics)` and moves `old` and the
  relativized path into it. All other `FormatSourceError` variants keep
  going to `FormatPathError::Format`.

- In both `FormatMode::Write` and `FormatMode::Check`, `format()` splits the
  collected errors into parse failures and everything else, then:

  1. Sorts the parse failures by path. Discovery runs in parallel, so without
     sorting the output order wouldn't be deterministic.

  2. Prints them with `print_parse_failures()` to a locked
     `termcolor::StandardStream::stderr()`.

  3. Logs the other errors with `tracing::error!`, as today.

  4. If any files failed to parse, logs one summary through
     `tracing::error!`: `Failed to parse 1 file.` or
     `Failed to parse 3 files.`.

  5. In check mode, prints the `Would reformat:` lines, as today.

- The exit status doesn't change. Any error still gives `ExitStatus::Error`.

### `stdin.rs`

- `FormatStdinError` gains `Parse(ParseFailure)`. `format_stdin()` takes
  the `--stdin-file-path` path, which is required for stdin, and uses
  `relativize_path()` on it for the header, so editors that pipe through
  stdin show a real file location.

- When parsing fails, it prints the failure and then
  `Failed to parse 1 file.`, matching the paths output.

### Log level

- The diagnostic frames are written straight to stderr, not through
  `tracing`, so `--log-level` doesn't filter them. They are the command's
  output, like `Would reformat:`. The summary line does go through
  `tracing`, so it does respect the log level.

## Tests

All as integration snapshots in `crates/air/tests/integration/format.rs`,
run with `--no-color`:

- One file, one error, in write mode. Replaces the snapshot for
  `stdin_errors_on_parse_error`, which moves to the new output.

- One file with several errors, to check that every one prints in source
  order.

- An error with a detail advice, like an unterminated string, to check that
  the second frame renders.

- Two broken files and one good file, passed in reverse alphabetical order,
  to check the sorted output and `Failed to parse 2 files.`.

- `--check` with one broken file and one that needs formatting, to check that
  the diagnostics come before `Would reformat:` and the exit code is an
  error.

- A CRLF file, to check that the frame has no stray `\r`.

- An error after a non-ASCII character on the same line, to check that the
  column counts characters.

- Stdin with `--stdin-file-path R/foo.R`, to check that the header shows
  `R/foo.R`.

- Use `.normalize_os_path_separator()` where a header has a nested path.

- Commit 1: a test with stderr piped and no `--no-color`, checking that the
  `ERROR` label and `Would reformat:` contain no ANSI codes. Integration
  tests always pipe stderr, so every existing snapshot already exercises the
  case without a terminal.

## CHANGELOG

- Under the CLI section: "Files that fail to parse now report each syntax
  error with its location and a code frame, instead of 'Failed to parse due
  to syntax errors.'"

- Under the CLI section: "Colored output is now turned off when stderr is not
  a terminal, so redirecting stderr to a file no longer writes ANSI codes."

## Decisions

- Render with `biome_diagnostics`, not `annotate-snippets`, so the CLI matches
  the corpus snapshots.

- Print every diagnostic, with no cap.

- Print the frames, then an `ERROR Failed to parse N file(s).` summary. The
  old per-file `ERROR Failed to format …` line is dropped for parse errors.

- Under stdin, the header shows the path from `--stdin-file-path`.

- Everything goes to stderr except formatted code. That includes the code
  frames, even under `--check`.

- Color is decided once, from stderr, `--no-color`, and `NO_COLOR`, then
  pushed into `colored` and `tracing-subscriber`.

- Use `PrintDiagnostic::simple()`, as biome's CLI does, in both the CLI and
  the corpus harness.

- Copy the source per diagnostic with `with_file_source_code()`, rather than
  writing a borrowing `Diagnostic` wrapper. It's simpler, and this only
  runs on failure.

## Out of scope

- Publishing parse diagnostics from the LSP.

- Formatting around local parse errors.

- A `--max-diagnostics` flag. If cascades from error recovery turn out to be
  noisy in practice, this is where we'd revisit it.
