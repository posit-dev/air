/// Integration tests for the air CLI
///
/// Directory structure inspired by:
/// https://matklad.github.io/2021/02/27/delete-cargo-integration-tests.html
///
/// Resolves problems with:
/// - Compilation times, by only having 1 integration test binary
/// - Dead code analysis of integration test helpers https://github.com/rust-lang/rust/issues/46379
mod format;
mod help;
mod helpers;
