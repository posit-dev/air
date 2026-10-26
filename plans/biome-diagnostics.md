# Biome diagnostics for user-facing errors

Status: draft, under review.

Supersedes commit 2 of `plans/diagnostics.md`. Commit 1 (#549) has landed.

## Goal

Every error the CLI shows a user is a `biome_diagnostics` diagnostic, printed
through `biome_console`'s `EnvConsole`. `tracing` goes back to being only a
log: trace, debug, info, and warn messages, controlled by `--log-level`.

Today:

```
ERROR Failed to read R/foo.R: Permission denied (os error 13)
ERROR Failed to format stdin: Failed to parse due to syntax errors.
Would reformat: R/bar.R
air failed
  Cause: Failed to parse /path/to/air.toml:
TOML parse error at line 2, column 14
...
```

After, for `air format --check .`:

```
R/foo.R internalError/io ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  × Failed to read file: stream did not contain valid UTF-8


format ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  × Found 1 error.


format ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  × 1 file would be reformatted.

  - R/bar.R

```

And for an invalid `air.toml`, which stops the command before any file is
formatted:

```
air.toml:2:14 configuration ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  × invalid type: string "a", expected an integer

    1 │ [format]
  > 2 │ line-width = "a"
      │              ^^^
    3 │


format ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  × Found 1 error.

```

## Where things stand

On `feature/diagnostics`, after the draft commit `e4d5e67f`:

- `format_source()` lives in `crates/air/src/commands/format/source.rs`, and
  `workspace` no longer formats.

- Parse failures render as code frames with `PrintDiagnostic::simple()`,
  through a hand-built `biome_console::fmt::Formatter` over
  `color::stderr()`.

- Every other error goes through `FormatPathError` or `FormatStdinError`, and
  is printed with `tracing::error!`. Both enums carry a `Parse` variant whose
  `Display` arm is never used.

- `Would reformat:` is written with `writeln!` and `colored`'s `underline()`.

- Config errors come back from `workspace` as `anyhow::Error`, and print as
  `air failed` followed by one `Cause:` line per error in the chain, from
  `main.rs`.

- `Can't supply paths when reading from stdin` is a `tracing::error!` in
  `commands/format.rs`.

- The project `CLAUDE.md` says CLI-facing errors are logged with
  `tracing::error!`. That rule changes with this plan.

## Facts that shape the design

- `EnvConsole::println()` and `print()` return `()` and `.unwrap()` every
  write. If stderr is a pipe whose reader has exited, as in
  `air format . 2>&1 | head`, the process panics with exit code 101. Every file
  has already been written by then, and nobody is reading the lost output. We
  accept this, and use `EnvConsole` directly.

- `console.error()` ends every message with a newline. `PrintDiagnostic`
  already ends with a blank line, so consecutive diagnostics are separated by
  one blank line, as in biome. Every frame snapshot from commit 2 gains these
  lines.

- Everything printed through a `Console` passes through `SanitizeAdapter`,
  which replaces every non-whitespace, zero-width character with U+FFFD. Biome
  2.4.5 corrupts a U+200B inside a string this way in `--stdin-file-path` mode.
  Formatted code, and shell completions, are never printed through the
  console. They keep their raw `stdout()` writes.

- `ColorMode::Auto` makes its own decision for stdout and stderr and defers to
  termcolor's environment checks. We always pass `ColorMode::Enabled` or
  `ColorMode::Disabled`, from the single decision in `color::init_color()`.

- Without color, `biome_console` swaps `✖` for `×`. With color, the header
  marker is `✖`. That's biome's behavior and we keep it.

- Categories are a fixed list compiled into `biome_diagnostics_categories`. An
  unregistered name is a `compile_error!`. Everything this plan needs exists in
  Lionel's fork: `parse`, `format`, `configuration`, `stdin`,
  `internalError/io`, `internalError/fs`, and `flags/invalid`.

- `biome_diagnostics::IoError` always carries `DiagnosticTags::INTERNAL`,
  which prints an ` INTERNAL ` badge in the header. A file we can't read is the
  user's problem, not an Air bug, so we don't use that adapter for read and
  write errors.

- `DiagnosticTags::INTERNAL` also adds the advice "This diagnostic was derived
  from an internal Biome error. Potential bug, please report it if necessary."
  That names Biome in Air's output, so we never use the tag.

- `biome_diagnostics::StdError` is behind the crate's `std` feature.

- `toml::de::Error` (toml 0.8) has `span()` and `message()`, so TOML syntax
  and type errors can point at the exact bytes in `air.toml`.

- `ExcludePatterns::try_from_iter()` returns `anyhow::Error`, with no span. Errors
  from `into_settings()` get the path but no frame.

## Design

### Output streams

- stdout carries formatted code in stdin mode, and shell completions. Nothing
  else.

- Everything else goes to stderr. Diagnostics go through `console.error()`, and
  logs through `tracing`.

- Biome prints its `Found N errors.` line to stdout. Our summary sections go
  to stderr, because stdout belongs to the formatted file.

### The console

- `run()` in `crates/air/src/lib.rs` builds one `EnvConsole` from the color
  decision, for every command except `language-server`, and passes
  `&mut dyn Console` into the command.

- `language-server` never gets a console. It sets up its own logging, and its
  stdout is the protocol channel.

- `color::init_color()` keeps its decision logic. It returns the `bool` that
  `tracing-subscriber` needs, and `run()` maps it to `ColorMode::Enabled` or
  `ColorMode::Disabled`. `color::stderr()` is deleted.

- `colored` is removed from the `air` crate once nothing uses it, along with its
  `set_override()` call.

### Logging

- `tracing` stays for logs: `trace!` for include and exclude decisions,
  `warn!` for things like the missing user configuration directory in
  `workspace/src/config.rs`, and so on.

- `--log-level` keeps its meaning and its default of `warn`. It never hides a
  diagnostic, and it never could hide an error, since `error` is its lowest
  level.

- No code in the `air` crate prints a user-facing error with `tracing::error!`.
  The LSP is out of scope and keeps its own logging.

- When we first emit a warning diagnostic, we add `--diagnostic-level`, like
  biome. Not before.

### Diagnostics

A new crate-level module, `crates/air/src/diagnostics.rs`, replaces
`commands/format/diagnostics.rs` and holds every diagnostic type plus the
printing helpers.

Each failure becomes a `biome_diagnostics::Error`, which is a boxed
`Diagnostic`:

| Failure | Category | Location | Frame | Tags |
| --- | --- | --- | --- | --- |
| Syntax error in an R file | `parse` | path and span | yes | none |
| Can't read an R file | `internalError/io` | path | no | none |
| Can't write an R file | `internalError/io` | path | no | none |
| `FormatError` or `PrintError` | `format` | path | no | none |
| Walk error with a path | `internalError/fs` | path | no | none |
| Walk error without a path | `internalError/fs` | none | no | none |
| Paths passed with `--stdin-file-path` | `flags/invalid` | none | no | none |
| Can't read stdin, or write stdout | `stdin` | none | no | none |
| Can't read `air.toml` | `internalError/io` | path | no | none |
| Invalid `air.toml` syntax or value | `configuration` | path and span | yes | none |
| Invalid setting, like a rooted `exclude` | `configuration` | path | no | none |

- Parse errors reuse `ParseDiagnostic` with `with_file_path()` and
  `with_file_source_code()`, as commit 2 does.

- The other types are small structs with `#[derive(Diagnostic)]`, with
  `severity = Error` written out, and messages that keep today's wording where
  there is any.

- Paths in headers go through `relativize_path()`. A file outside the working
  directory, such as a user-level `air.toml`, keeps its absolute path, and gets
  an OSC 8 link when color is on. That's accepted, as in the original plan.

- Files that would be reformatted under `--check` aren't diagnostics of their
  own. They're listed in the reformatted summary, described below.

- `FormatPathError`, `FormatStdinError`, `ParseFailure`, and the dead `Parse`
  `Display` arms are deleted. The functions return `biome_diagnostics::Error`.

### Ordering and the summary

- The format command collects every diagnostic, then prints them all at the
  end, in one pass.

- Diagnostics are stably sorted by their file path, with diagnostics that have
  no path first. Discovery runs in parallel, so without sorting the order isn't
  deterministic. A stable sort keeps one file's diagnostics in source order.

- After the diagnostics, the command closes with summary diagnostics, like
  biome's closing `format` diagnostic. Each has the `format` category, no
  location, and `severity = Error`, so it renders as its own `format ━━━`
  section:

  - `Found 1 error.` or `Found N errors.`, if any diagnostic is an error.

  - `1 file would be reformatted.` or `N files would be reformatted.`, if any
    file would be reformatted under `--check`, followed by a list advice of
    their paths, sorted and relativized, with no hyperlinks.

- When both apply, both sections print, errors first.

- N counts diagnostics, not files. A file with three syntax errors adds three.

- Would-be-reformatted files don't count toward `Found N errors.`, so
  the summary matches the exit code: a run that exits with `1` never says it
  found errors.

- The summary diagnostics are printed after the sort, and aren't part of the
  counts or the exit status.

- Stdin uses the same `Found N errors.` section. It never prints the
  reformatted section, since stdin `--check` stays silent.

- Exit codes don't change:

  - `ExitStatus::Error` if there are any diagnostics.

  - `ExitStatus::Failure` if there are none, but some file would be
    reformatted.

  - `ExitStatus::Success` otherwise.

### Stdin

- Parse errors use the `--stdin-file-path` for the header, as in commit 2.

- `FormatError` and `PrintError` also use that path.

- A failure to read stdin or write stdout uses the `stdin` category, with no
  location.

- `--check` with stdin that needs formatting stays silent and exits with
  `ExitStatus::Failure`, as it does today.

- The formatted output is written with `stdout().lock().write_all()`, never the
  console.

### Configuration errors

- `workspace` replaces `anyhow` in `discover_settings()`,
  `discover_user_settings()`, and `parse_settings()` with structured errors.
  `ParseTomlError` in `crates/workspace/src/toml.rs` becomes `TomlError`,
  and `crates/workspace/src/discovery.rs` owns `DiscoverSettingsError`, with a
  manual `From<TomlError>` conversion:

  ```rust
  pub enum TomlError {
      Read { path: PathBuf, error: io::Error },
      Deserialize { path: PathBuf, contents: String, error: Box<toml::de::Error> },
  }

  pub enum DiscoverSettingsError {
      Read { path: PathBuf, error: io::Error },
      Deserialize { path: PathBuf, contents: String, error: Box<toml::de::Error> },
      Logic { path: PathBuf, error: anyhow::Error },
  }
  ```

- `Display` prints exactly what the `anyhow` chain prints today, so the LSP
  compiles and logs unchanged. Its call sites only use `{error}`.

- The CLI turns `Deserialize` into a `configuration` `SettingsDiagnostic`, with
  `error.message()` as the message and `error.span()` as the location over the
  `air.toml` source.

- `Logic` keeps `anyhow::Error` inside, since `ExcludePatterns` returns it.
  Giving it spans, for example with `toml::Spanned`, is out of scope.

- A configuration error ends the command with `ExitStatus::Error`, after
  printing the diagnostic and the summary.

### No unexpected errors

- After the configuration PR, no command can return an `anyhow::Error`:

  - `format()` reports every failure as a diagnostic.

  - `generate_shell_completion()` never fails, since `clap_complete` panics
    on a failed write.

  - `language_server()` never fails, since `lsp::start_lsp()` returns `()`.

- So there's no safety net. `run()` and every command return `ExitStatus`
  directly, and `main.rs` only maps it to an `ExitCode`, without
  `air failed` and `Cause:`.

- A new failure gets its own diagnostic, not a path through `anyhow`.

## Pull requests

A stack of four, built with `gh stack`. The first one replaces the current
draft on `feature/diagnostics`. Branch names are proposals.

1. `feature/diagnostics`: the console, and diagnostics for the format command.

   - Builds `EnvConsole` in `run()` and passes it to the format command.

   - Moves the diagnostics module to the crate level.

   - Converts parse, read, write, format, print, walk, stdin, and
     argument errors to diagnostics.

   - Adds the sort and the `Found N errors.` summary section.

   - Keeps `Would reformat:` as it is, printed with `writeln!` to stderr.

2. `feature/diagnostics-check`: `--check` lists would-be-reformatted files in a
   `format` summary diagnostic, and `colored` is removed.

3. `feature/diagnostics-config`: structured `TomlError` and
   `DiscoverSettingsError` in `workspace`, and configuration diagnostics with
   frames into `air.toml`.

4. `feature/diagnostics-fatal`: commands return `ExitStatus` instead of
   `anyhow::Result`, and `main.rs` loses `air failed`.

Each PR updates `CHANGELOG.md`, its snapshots, and its tests, and passes
`just test`, clippy, and `cargo fmt --check` on its own.

## Tests

All integration tests run with `--no-color` and keep raw bytes, trailing
whitespace included.

- PR 1:

  - The existing parse error snapshots, which gain blank lines between
    diagnostics and end with a `format` section saying `Found N errors.`

  - `air format missing.R`, for a walk error with a path. The OS text differs
    on Windows ("The system cannot find the file specified."), so this
    snapshot needs an insta filter or a `#[cfg(unix)]`.

  - A file that can't be read. A file containing invalid UTF-8 is portable:
    `read_to_string()` fails with "stream did not contain valid UTF-8" on
    every platform.

  - Parse errors in two files plus a read error, to check the sort and that
    the total counts all three diagnostic sources.

  - `test_stdin_cant_supply_paths`, which moves to the `flags/invalid`
    diagnostic.

  - Stdin with U+200B inside a string, comparing stdout byte for byte with the
    input, so the formatted output can never go through the console.

  - `test_no_color_when_stderr_is_not_a_terminal`, which still asserts no
    `\x1b` in stderr.

- PR 2:

  - `test_check_output_format` and the other `--check` snapshots, moved to
    the reformatted summary.

  - `--check` with a broken file and a file that needs formatting: the parse
    diagnostic, then both summary sections, errors first, and exit code 255.

  - `--check` with files passed in reverse order, to check the list is sorted.

  - `--check` where every file only needs formatting: exit code 1, a
    `N files would be reformatted.` section, and no `Found N errors.` section.

- PR 3:

  - A TOML syntax error in a project `air.toml`, with a frame.

  - A type error, like `line-width = "a"`, with a frame under the value.

  - A rooted `exclude` in a user-level `air.toml`, with no frame.

  - The existing `workspace` tests that assert on error text keep passing,
    because `Display` doesn't change.

- PR 4:

  - None, since PR 4 adds no behavior. The existing tests cover `run()`.

## Documentation

- `CHANGELOG.md`, under the development version:

  - "Errors are now reported as diagnostics. Files that fail to parse show each
    syntax error with its location and a code frame." (PR 1)

  - "`air format --check` now reports files that would be reformatted as
    diagnostics." (PR 2)

  - "Errors in `air.toml` now point at the exact line and column, with a code
    frame." (PR 3)

- The project `CLAUDE.md`: replace "CLI-facing errors: log with
  `tracing::error!`" with the new rule, in PR 1.

## Out of scope

- `--diagnostic-level` and `--max-diagnostics`.

- Publishing `air.toml` or parse diagnostics from the LSP.

- Spans for errors raised after TOML deserialization, like rooted `exclude`
  patterns.

- A diff for files that would be reformatted, like biome's
  `FormatDiffDiagnostic`.

- Reporting the U+FFFD corruption in biome's stdin mode upstream.

## Decisions

- Use `EnvConsole` directly, and accept its panic on a closed stderr.

- Keep biome's category names, from the fixed list in Lionel's fork.

- `tracing` stays for logs, with `--log-level` defaulting to `warn`.

- The run ends with summary diagnostics in the `format` category, with no
  location: `Found N errors.`, then `N files would be reformatted.`, each only
  when it applies.

- Would-be-reformatted files don't count toward `Found N errors.`

- Would-be-reformatted files are a list advice on the reformatted summary, not
  one diagnostic each, to keep `--check` on an unformatted project short.

- No diagnostic carries `INTERNAL`, since its advice names Biome.

- No `anyhow` safety net in `run()`, since nothing can reach it.

- Stdin `--check` stays silent when the input needs formatting. Editors and
  scripts use the exit code.

- `air.toml` errors become diagnostics with frames. The LSP keeps logging the
  same `Display` text.

- This replaces commit 2 of `plans/diagnostics.md` on `feature/diagnostics`.

- `language-server` gets no console.

- No clippy rule against `tracing::error!`. The project `CLAUDE.md` carries the
  rule instead.

## Checklist

### PR 1: the console, and diagnostics for the format command

- [x] Ask before creating or rewriting any branch. Never commit, rebase, or
      open a PR.

- [x] Add the `std` feature to `biome_diagnostics` in `crates/air/Cargo.toml`.
      Skipped, since nothing uses `StdError` or `IoError`.

- [x] Build `EnvConsole` in `run()` from `color::init_color()`, mapping the
      `bool` to `ColorMode::Enabled` or `ColorMode::Disabled`.

- [x] Pass `&mut dyn Console` from `run()` into `commands::format::format()`,
      then into `paths::format()` and `stdin::format()`.

- [x] Delete `color::stderr()`, and update the `init_color()` doc comment.

- [x] Move `commands/format/diagnostics.rs` to `crates/air/src/diagnostics.rs`.

- [x] Add the diagnostic types from the table for read, write, format, print,
      walk, stdin, and argument errors.

- [x] Make `format_path()` and `format_stdin()` return
      `biome_diagnostics::Error`, and delete `FormatPathError`,
      `FormatStdinError`, and `ParseFailure`. They return `Vec<Error>`
      instead, since a file can have many syntax errors.

- [x] Turn `check_argument_consistency()` into a `flags/invalid` diagnostic.

- [x] Add the stable sort by path, pathless first.

- [x] Print every diagnostic with
      `console.error(markup! {{PrintDiagnostic::simple(&diagnostic)}})`.

- [x] Add the summary diagnostic type: `format` category, no location,
      `severity = Error`.

- [x] After the sorted diagnostics, print a `Found 1 error.` or
      `Found N errors.` summary diagnostic when there are errors, in both
      paths and stdin mode.

- [x] Keep the exit code rules, and keep `Would reformat:` as it is.

- [x] Confirm formatted stdin output still uses a raw `stdout()` write.

- [x] Confirm no `tracing::error!` is left in `crates/air/src`.

- [x] Add and update the PR 1 tests, then review every changed snapshot by
      hand.

- [x] Update `CHANGELOG.md` and the project `CLAUDE.md`.

- [x] Run `just test`, `cargo clippy --workspace --all-targets`, and
      `cargo fmt --all -- --check`.

### PR 2: `--check` diagnostics

- [x] Replace `inform_changed()` with a list advice on the reformatted summary
      diagnostic, instead of a `format` "File would be reformatted." diagnostic
      per file. Paths are sorted and plain, with no hyperlinks, as on `main`.

- [x] Keep `ExitStatus::Failure` when reformatted files are the only problem.

- [x] Leave them out of `Found N errors.`, and print a `1 file would be
      reformatted.` or `N files would be reformatted.` summary diagnostic
      after the errors section. `print_diagnostics()` takes the reformatted paths
      as a second argument.

- [x] Remove `colored` from `crates/air/Cargo.toml`, and its `set_override()`
      call. `itertools` goes too, along with `colored` in the workspace
      `Cargo.toml`.

- [x] Add and update the PR 2 tests.

- [x] Update `CHANGELOG.md`. Skipped.

- [x] Run `just test`, clippy, and `cargo fmt --check`.

### PR 3: configuration diagnostics

- [x] Replace `anyhow` in `discover_settings()`, `discover_user_settings()`,
      and `parse_settings()` with `DiscoverSettingsError`, and replace
      `ParseTomlError` with `TomlError`.

- [x] Keep the source text of `air.toml` in the `Deserialize` variant.

- [x] Keep `Display` byte-for-byte identical to today's messages.

- [x] Check that every LSP call site in `crates/lsp/src/workspaces.rs`
      compiles unchanged and logs the same text. One needs `error.into()`,
      since its closure also takes the `anyhow::Error` from `url_to_path()`.

- [x] Make `resolve_settings()` return `DiscoverSettingsError`, and turn it
      into diagnostics with `from_discover_settings_error()`.

- [x] Add the PR 3 tests.

- [ ] Update `CHANGELOG.md`.

- [x] Run `just test`, clippy, and `cargo fmt --check`.

### PR 4: no unexpected errors

- [x] Drop the safety net. `run()`, `generate_shell_completion()`, and
      `language_server()` return `ExitStatus` instead of
      `anyhow::Result<ExitStatus>`.

- [x] Reduce `main.rs` to mapping `ExitStatus` to `ExitCode`.

- [x] Remove `tags(INTERNAL)` from `FormatDiagnostic`.

- [x] Update the project `CLAUDE.md`, since commands no longer return
      `anyhow::Result`.

- [x] Run `just test`, clippy, and `cargo fmt --check`.
