# Air

## Project Overview

Air is an R language server and formatter written in Rust. Workspace crate layout under `crates/` with a CLI binary in `crates/air/`.

- Edition 2024, Rust 1.87+
- Uses `biome_formatter` for the formatting engine and `ignore` crate for file walking
- Config file: `air.toml` discovered by walking up directory ancestors
- Snapshot testing with `insta`

## Code Style

### Imports

- One item per `use` line, no `{}` grouping
- Grouped in order: `std`, external crates, `crate`/`super`, with groups separated by blank lines

### Error Handling

- `thiserror::Error` for structured error enums; manual `Display` impls when messages need custom formatting (e.g. path underlining)
- CLI-facing errors: log with `tracing::error!` and return `Ok(ExitStatus::Error)`. Reserve `anyhow::Error` for truly unexpected errors.
- `tracing::trace!` for file inclusion/exclusion decisions

### Functions

- Normalize paths with `fs::normalize_path` at function boundaries
- Prefer returning structured enums (e.g. `FormattedSource::Changed`/`Unchanged`) over booleans

### Comments

- Explain the "why", not the "what"
- Reference GitHub issues when behavior is motivated by a specific bug report

## Testing

### Running Tests

- Use `just test` to run tests via nextest, NOT `cargo test`
- Example: `just test -p workspace --lib discovery::test`
- Passes arguments through to `cargo nextest run`

### Unit Tests

- `#[cfg(test)] mod test` at the bottom of the source file
- Use `tempfile::TempDir`, immediately shadow: `let tempdir = TempDir::new()?; let tempdir = tempdir.path();`
- Use `anyhow::Result<()>` as test return type
- Write files into temp dirs to set up test fixtures inline

### Integration Tests

- Live in `crates/air/tests/integration/`
- `insta::assert_snapshot!` for CLI output assertions
- Build `Command` with method chaining: `.current_dir()`, `.arg()`, `.run()` or `.run_with_stdin()`
- `.remove_arguments()` when snapshot would contain machine-specific absolute paths
- `.normalize_os_path_separator()` for cross-platform path assertions
- Test both the happy path and the exclusion/rejection path
