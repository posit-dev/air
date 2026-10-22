# Moving air onto aether

Status: ready to start the swap. One open item at the bottom.

## Goal

Bring all of aether into air under the `air_` prefix. Aether's crates replace
air's crates of the same role, and aether's test crates come along unchanged.
After the swap, air's parser is aether's hand-written parser, and tree-sitter is
gone.

This is a plain copy, not a dependency. Air does not pin aether by git, and
there is no `aether_r_*` rename across air's code. If the parser ever needs to
live on its own again, it can be split back out.

Success means:

- `crates/air_r_parser` is aether's parser, and `tree-sitter` and
  `tree-sitter-r` are gone from the workspace.

- `air_r_parser_tests` and `air_r_test_macros` exist, and `just accept` and
  `just test-quick-parser` work.

- Every formatter spec snapshot is unchanged, except for new specs covering
  input that only aether can parse.

## Rule for collisions

When air and aether disagree on a file, crate, setting, or dependency, aether
wins. Exceptions are listed under Decisions.

## Where the two repos stand

The prep work (P1 to P7, all merged) has brought air's tree-sitter walker in
line with aether's tree shape. What's left to reconcile is small:

- `r.ungram`: byte-identical.

- `air_r_syntax` vs `aether_r_syntax`, after swapping the prefix: aether adds
  `string_ext.rs` and `pub use biome_rowan::TokenText`. It also splits one
  `use crate::{RRoot, RSyntaxKind}` into two lines.

- `air_r_factory` vs `aether_r_factory`: identical after swapping the prefix.

- `biome_ungrammar`: identical, except air has a stray `Cargo.lock`.

- `air_r_parser` vs `aether_r_parser`: same public API (`RParse`, `parse()`,
  `RParserOptions`, `has_errors()`, `diagnostics()`, `into_diagnostics()`,
  `From<RParse> for AnyParse`). The internals are fully replaced.

- `xtask`: aether's is a superset of air's. It adds `accept.rs`, a `main.rs`,
  a `bpaf` `TaskCommand`, and a `project_root()` that searches for
  `[workspace]`.

- `xtask/codegen`: aether's is air's minus `formatter.rs`, the `Formatter` and
  `All` commands, and `LanguageKind::formatter_crate_name()`.

- Air's `tests_macros` stays. The formatter spec harness still uses it.
  `air_r_test_macros` is a different, smaller macro for the markdown corpus.

## The stack

Two prep PRs on `main`, then one swap PR.

### Prep 1: drop the tree-sitter LSP view

Self-contained, and it can land now.

- `crates/lsp/src/handlers_ext.rs`: remove `ViewFileKind::TreeSitter` and
  `format_ts_node()`.

- `crates/lsp/Cargo.toml`: drop `tree-sitter` and `tree-sitter-r`.

- `editors/code`: remove `air.viewTreeSitter` from `commands.ts`,
  `package.json`, and the `"TreeSitter"` member of `ViewFileKind` in
  `request/viewFile.ts`.

- Changelog bullet for the VS Code extension.

### Prep 2: `test-health.yml` with fmt and clippy

Air's CI runs neither today.

- New `.github/workflows/test-health.yml`, wired into `test.yml`. It runs
  `cargo fmt --all --check` and
  `cargo clippy --workspace --all-targets -- -D warnings` on Linux.

- Fix whatever they flag in air's code, in its own commit.

- Take aether's `rust-toolchain.toml`, which adds
  `components = ["rustfmt", "clippy"]`.

- Rename the clippy lint `empty_enum` to `empty_enums`, as aether does.

The swap PR adds two more checks to the same workflow.

### The swap PR

Each bullet is roughly one commit.

- [ ] Copy aether's crates in, renaming `aether` to `air` in crate names,
  paths, idents, and descriptions:
  - `aether_r_parser` replaces `crates/air_r_parser` wholesale, including
    deleting `treesitter.rs`, the old spec harness, and the 78 fixtures under
    `tests/snapshots/`. Aether's corpus replaces them.
  - `aether_r_syntax` replaces `crates/air_r_syntax`.
  - `aether_r_factory` replaces `crates/air_r_factory`.
  - `aether_r_parser_tests` arrives as `crates/air_r_parser_tests`.
  - `aether_r_test_macros` arrives as `crates/air_r_test_macros`.
  - `biome_ungrammar` replaces air's copy, dropping the stray `Cargo.lock`.

- [ ] `xtask`: aether's `xtask` replaces air's, renamed. Add `"xtask"` to
  workspace `members`, since it now has a binary.

- [ ] `xtask/codegen`: start from aether's crate, then re-add air's
  `formatter.rs` unchanged (`git2` included), the `Formatter` and `All`
  commands, and `formatter_crate_name()`. Run `just gen-grammar` and
  `just gen-formatter`. Both should produce no diff.

- [ ] Workspace `Cargo.toml`:
  - Add `air_r_parser_tests`, `air_r_test_macros`, `biome_console`,
    `biome_diagnostics`, `bpaf`, `similar`, and `unicode-ident`.
  - Give `serde` the `derive` feature, as aether does.
  - Remove `tree-sitter` and `tree-sitter-r`, plus `biome_unicode_table` if
    nothing else uses it.

- [ ] `.config/nextest.toml`: take aether's, verbatim.

- [ ] `justfile`:
  - `gen-grammar` stays, and runs the merged codegen.
  - Aether's `test` is dropped. Air's `test *ARGS` stays.
  - Aether's `accept` is added as-is.
  - Aether's `quick` is added as `test-quick-parser`.
  - Air's `test-quick` becomes `test-quick-formatter`.

- [ ] `test-health.yml`: add aether's two checks, which are
  `just gen-grammar && git diff --exit-code` and
  `cargo run -p xtask -- accept && git diff --exit-code`.

- [ ] Formatter specs for semicolons, replacing the hand-built `TODO(aether)`
  tests in `r/auxiliary/empty_statement.rs`. New `statements/semicolons.R`
  covers:
  - `1;2`, `1;`, `1;;2` in braces, `{;}`, `{;1}`.
  - `1; # c` and `1 ; # c`.
  - A comment before a `;` on the next line, as in `{1 # c` then `;}`.
  - Blank lines around a lone `;` in braces.
  - `# fmt: skip` on a statement ending in `;` (kept verbatim, `;` included).
  - Curly-curly with a `;`: `fn({{ x }; })` and `fn({{ x; }})` both format
    to `fn({{ x }})`.
  - Every case from the hand-built tests.

- [ ] Formatter specs for input only aether can parse: `r"-|a|-"` raw strings,
  `%a\b%`, and `for (... in x)`.

- [ ] Range formatting across `;` in the LSP tests.

- [ ] Snapshots: the `crates/crates` crate list and the LSP `document_syntax`
  snapshots.

- [ ] `CLAUDE.md`:
  - Root: update the crate layout, say that the parser is hand-written, and
    document `just accept` and `just test-quick-parser`.
  - New nested `crates/air_r_parser/CLAUDE.md`, a trimmed copy of aether's. It
    keeps the invariants, the corpus test format, and the reference
    implementations. It drops the `deviations.md` references, since that file
    doesn't come over.

- [ ] Docs: `CONTRIBUTING.md`, `xtask/README.md`, `xtask/codegen/README.md`.

- [ ] `CHANGELOG.md`:
  - Files R rejects are now rejected: two expressions on one line, a stray
    top-level `;`, `pkg::` followed by a newline, `x$1`.
  - `r"-|...|-"`, `%a\b%`, and `for (... in x)` now parse.
  - Building air no longer needs a C compiler.
  - Performance, if the benchmark shows a win.

- [ ] Validation before merge:
  - Ecosystem diff: format a fixed list of GitHub repos, cloned at pinned SHAs,
    with `main` and with the branch, then diff. Every formatting difference must
    be explained, and every file that newly fails to parse must be one R also
    rejects.
  - CLI benchmark on the same corpus.

## Decisions

- Plain copy, not a git subtree. Blame for parser files starts at the import
  commit.

- One swap PR after two prep PRs, not a longer stack.

- Aether's `docs/deviations.md` does not come over.

- Aether's `CLAUDE.md` becomes a nested `crates/air_r_parser/CLAUDE.md`.

- `formatter.rs` comes over unchanged, `git2` and all.

- Aether's `.config/nextest.toml` is taken verbatim, including the 3×10s
  terminate. Any LSP test that trips it gets fixed.

- The CLI keeps today's one-line parse error. Rich diagnostics are a follow-up.

- What happens to the aether repo is out of scope.

## Follow-ups after the swap

- Print parse diagnostics with locations from the CLI, instead of "Failed to
  parse due to syntax errors." This is the `TODO(aether)` in
  `crates/workspace/src/format.rs`.

- Consider publishing parse diagnostics from the LSP, now that they carry
  ranges.

- Consider formatting around local errors, since the parser keeps the rest of
  the tree intact. Air refuses today, and that stays true in this plan.

## Open

1. The repo list for the ecosystem diff. It needs to go here before the swap PR
   is opened.
