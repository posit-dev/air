# P6: Statement layer without semicolons

Branch: `feature/statements`, on top of `main` at `628bed35`. Nothing is committed.

This is P6 from `plans/aether.md`. Each expression at the top level and inside `{}` is now wrapped in an `RStatement`. The `;` slot exists, but nothing fills it yet: tree-sitter-r hides semicolons, so the walker can't emit them. Semicolons arrive with the swap to aether.

Expected result: zero formatter snapshot diff. That is what we got.

## Grammar and codegen

- `xtask/codegen/r.ungram` and `xtask/codegen/src/r_kinds_src.rs` now match aether's exactly (verified with `diff`):

  - `RRoot.expressions: RExpressionList` became `RRoot.statements: RStatementList`.

  - `RBracedExpressions` became `RBracedStatements`, and its `expressions` field became `statements`.

  - `RExpressionList = AnyRExpression*` became `RStatementList = RStatement*`.

  - New `RStatement = expression: AnyRExpression? semicolon: ';'?`.

- `crates/air_r_syntax/src/lib.rs`: `is_bogus()` and `to_bogus()` arms are updated. They match aether's `lib.rs`, apart from aether's `string_ext` and `TokenText` export.

- I ran `cargo run -p xtask_codegen -- grammar r`. After replacing `air_r_` with `aether_r_`, the output in `air_r_syntax/src/generated/*` and `air_r_factory/src/generated/*` is byte-identical to aether's.

- I `git mv`'d two formatter files before running formatter codegen, so the generator wouldn't delete them:

  - `r/auxiliary/braced_expressions.rs` to `r/auxiliary/braced_statements.rs`

  - `r/lists/expression_list.rs` to `r/lists/statement_list.rs`

- Then I ran `cargo run -p xtask_codegen -- formatter`. It regenerated `generated.rs`, `r/any/expression.rs` and the `mod.rs` files, and created the `r/auxiliary/statement.rs` stub. The codegen staged some of its outputs in the git index.

## Walker (`crates/air_r_parser`)

- `parse.rs`: there is a new `handle_statement()`, which wraps the next child in an `R_STATEMENT` and walks it. Two places call it for every non-comment child:

  - `handle_root_enter()`, which now loops over the program's children like the braced handler does.

  - `handle_braced_statements_enter()`, which was renamed from `handle_braced_expressions_enter()`.

- Comments stay outside statements. That doesn't matter, because trivia attaches to tokens, not to Start/Finish events.

- `parse_failure()` now emits `R_ROOT > R_STATEMENT_LIST > R_STATEMENT > R_BOGUS_EXPRESSION`.

- The unit tests that check walker events now expect `R_STATEMENT_LIST` and `R_STATEMENT`.

- `treesitter.rs`: `"braced_expression"` now maps to `R_BRACED_STATEMENTS`.

- `;` is still consumed as whitespace trivia. The `TODO(semicolon)` comments are still accurate.

## Formatter (`crates/air_r_formatter`)

### Why anything beyond renames was needed

Biome attaches a comment to the outermost node that starts or ends next to it. That node is now the `RStatement`, not the expression. So own-line leading comments, and trailing comments at statement level, now live on the statement. Any code that looked up comments on a top-level or braced expression, or checked that an expression's parent is the list, had to change.

### Changes

- `r/auxiliary/statement.rs`: `FormatRStatement` formats `expression`, then calls `format_removed()` on `semicolon_token` if it is present.

  - Nothing in P6 can reach that `;` branch.

  - It uses the default `is_suppressed()`, so `# fmt: skip` now suppresses the whole statement, which is the decision recorded in the plan.

- `r/lists/statement_list.rs`: only the type is renamed. It is still a hardline join with `EmptyLines`.

- `r/auxiliary/root.rs`: `statements` replaces `expressions`. The `is_suppressed()` check for `# fmt: skip file` reads the leading comments of the first statement.

- `comments_ext.rs` (directives):

  - `can_have_directive()` was replaced by `directive_node()`. If a node's parent is an `R_STATEMENT`, its directives are read from the statement.

  - Otherwise, the parent must be `R_STATEMENT_LIST`, `R_ARGUMENT_LIST` or `R_BINARY_EXPRESSION`, as before with `R_EXPRESSION_LIST`.

  - This keeps `# fmt: table` working on top-level and braced calls (`call.rs` `is_table()`) and assignments (`binary_expression.rs` `fmt_binary_assignment()`). Those functions query the expression, but the comment is now on the statement.

- `comments.rs`: `place_leading_or_dangling_body_comment()` makes the comment lead the first statement of the body.

- `r/auxiliary/binary_expression.rs`: in `can_normalize_to_equal()`, the "top level or braced" check is now `RStatement::can_cast(parent_kind)`.

- `r/auxiliary/if_statement.rs`: `SyntaxPositionExt::position()` now steps through the statement layer: from the `RStatement` parent, to the `RStatementList`, to either `RRoot` or the braced block. The value case compares against the last statement.

- `r/auxiliary/braced_statements.rs`, curly-curly:

  - `RCurlyCurly`'s `node_inner: RBracedStatements` field was replaced by `statement_inner: RStatement`, which wraps the inner `{`. `expression` was renamed to `statements`.

  - `fmt_curly_curly()` skips formatting both `statement_inner` and the inner `{`, so it marks both as suppression checked. The inner `{` comes from `statement_inner.expression()`.

  - Comments only need formatting on `statement_inner`. It has the same range as the inner `{`, and Biome attaches comments to the outermost node, so nothing can attach to the inner `{`. Removing the old leading and trailing comment calls on it changed no tests.

  - There is no `;` handling. In the swap, a `;` disqualifies curly-curly (`plans/aether.md`).

### Renames, for consistency only

- `BracedExpressionsVariant` and `braced_expressions_variant()`

- `braced_expressions_is_groupable()`

- The if-statement enum `BracedExpressions { Force, IfGroupBreaks }`, with its fields and `compute_braced_expressions()`

- `should_force_braced_expressions()`

All of these are now `*statements*`. Davis may veto the enum rename as churn.

## LSP (`crates/lsp/src/handlers_format.rs`)

Range formatting now:

- Finds `R_STATEMENT_LIST` parents, in `find_statement_lists()`, which was renamed.

- Casts the logical lines to `RStatement`.

- Builds the synthetic root from `r_statement_list()`.

## Snapshots

- 39 parser snapshots in `crates/air_r_parser/tests/snapshots/**` changed, and 2 LSP snapshots (`lsp__documents__tests__document_syntax*.snap`).

- A script compared each old and new CST:

  - The token streams (text, ranges, trivia) are identical.

  - The `## Errors` sections are identical.

  - The only node changes are the renames plus added `R_STATEMENT` nodes.

- No formatter snapshots changed.

## Verification

- `just test --workspace`: 243 passed.

- `cargo clippy --workspace --all-targets` is clean, apart from a pre-existing `clippy::empty_enum` rename warning.

- `cargo fmt --check`: my files are clean. The remaining diffs in `crates/lsp` (import order) were already on `main`.

- Corpus check on 11,502 `.R` files from `~/files/programming/oak-r-sources`, run with `air format --force --no-configuration`:

  - The release build of `main` (built from `git archive HEAD`) and the release build of the branch produce byte-identical output. Formatting changed 11,323 of the files, so this exercised real formatting.

  - The debug build of the branch, which runs Biome's comment and suppression assertions, produced no panics. Its output is identical too.

## Coverage of the risky paths

Existing specs already cover these, and all of them are unchanged:

- Leading and trailing comments on the skipped curly-curly nodes: the "Comprehensive comment test" in `braced_expressions.R`.

- `# fmt: table` at the top level, in braces, and on an assignment: `call_table.R`.

- `# fmt: skip` at the top level and in braces: `directives/skip.R`.

- The value and effect cases of `position()`: "Auto bracing" in `if_statement.R`.

- Range formatting across nested lists: `test_format_range_unmatched_lists`.

Nothing tests the `;` branch in `FormatRStatement`. It can't be reached until the swap, which adds `statements/semicolons.R`. That file's checklist includes `fn({{ x }; })` and `fn({{ x; }})`, which must not format as curly-curly.

## Changes to `plans/aether.md`

- P6 is marked done.

- The `git mv` hazard note now says P6 did both renames. The `git mv` step was removed from the swap's `xtask` item.

- The swap's semicolon item now says `format_removed()` already exists. It also adds a new finding: in `{ if (a) 1;; }` the last statement is empty, so `position()` must skip trailing expression-less statements.

- A new swap item says a `;` in either statement disqualifies curly-curly.

## Out of scope

- No CHANGELOG entry, because nothing changes for users.

- Empty statements, and placing comments on them, are left for the swap.

## Later: `RStatement` split

`RStatement` was later split into `AnyRStatement = RExpressionStatement | REmptyStatement` (P6b), which also added the empty-statement formatting. See `plans/empty-statement.md`. Its "Outcome in air" section corrects one claim above: `# fmt: skip` suppressed only the expression in P6, not the whole statement.
