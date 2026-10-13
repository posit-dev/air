# Moving air onto aether

Status: draft, open questions at the bottom.

## Goal

Replace air's tree-sitter based `air_r_parser`, and the `air_r_syntax` /
`air_r_factory` crates generated from air's own `r.ungram`, with aether's
`aether_r_parser`, `aether_r_syntax`, and `aether_r_factory`. After the move,
air owns no grammar and no parser. It owns the formatter, the LSP, the CLI, and
a formatter-only codegen.

Success means:

- `air_r_parser`, `air_r_syntax`, `air_r_factory`, `biome_ungrammar`,
  `tree-sitter`, and `tree-sitter-r` are gone from air's workspace.

- Every formatter spec snapshot is unchanged, except where a deviation in
  aether's `docs/deviations.md` explains the change.

- `xtask` generates formatter boilerplate only, reading the grammar from aether
  instead of a local copy.

## Short answer: piecewise or one big PR?

Mostly piecewise. Nearly every tree-shape deviation can be reproduced by air's
own tree-sitter walker first. That lets each formatter change land on `main`
against a green snapshot suite, one deviation at a time. The swap PR at the end
is then mostly mechanical.

What can't be split out of the swap:

- The dependency swap itself, crate deletions, and the `air_r_*` to `aether_r_*`
  rename.

- Semicolons as real `SEMICOLON` tokens. tree-sitter-r never emits `;`, so
  faking it in the walker means synthesizing tokens from gap text. That's real
  throwaway complexity for a small formatter change, so it stays in the swap.
  The statement layer around it does not (see P6).

- Diagnostics-only deviations (`1 2`, a stray top-level `;`, `foo::\nbar`,
  `a$1`, and so on). These only exist in aether's parser.

- `|` raw strings and `%a\b%`, which tree-sitter can't lex.

- Rewiring `xtask` to read aether's grammar.

## Findings from research

### Experiments run

I parsed every `.R` fixture in air with aether, using a scratch probe that
depends on `aether_r_parser` by path.

- Formatter specs: exactly one fixture fails. `binary_expression_sticky.R` line
  41 is two expressions glued onto one line, a typo in the fixture. tree-sitter
  accepted it silently. After the swap, the spec harness would panic on it.

- Parser fixtures: the expected `error/` failures, plus `ok/semicolons/*` (stray
  top-level `;`, now an error as in R) and `undefined/*` (now diagnosed). These
  fixtures are deleted with `air_r_parser` anyway.

I also ran today's `air` binary on semicolon inputs:

  | input                       | air today                                             | under aether                           |
  | ---                         | ---                                                   | ---                                    |
  | `1;2`                       | `1` / `2` on two lines                                | same, by removing the `;` token        |
  | `{1; 2}`                    | two lines in braces                                   | same                                   |
  | `1; # c`                    | `1 # c`                                               | same, if comments are placed correctly |
  | `;1`                        | `1`                                                   | parse error, refuses to format         |
  | `1 2`                       | `1` / `2`                                             | parse error, refuses to format         |
  | `for (... in 1:2) print(1)` | verbatim, trailing newline lost (whole file is bogus) | formats normally                       |

So for valid R, the semicolon formatting that was agreed on in `deviations.md`
gives the same output air produces today. The semicolon work is about keeping
output identical, not changing it. Air has no semicolon formatter specs at all,
which needs fixing.

### The `RStatement` layer is the subtle part

Today an expression's parent is `RExpressionList`. After the swap it is
`RStatement`, and the list is one level further up. Biome attaches a leading
comment to the outermost node that follows it, so own-line comments become
leading comments of the `RStatement`, not of the expression. Places that break:

- `comments_ext.rs` `can_have_directive()` matches the parent kind
  `R_EXPRESSION_LIST`. `# fmt: skip` on a top-level or braced expression would
  silently stop working.

- `if_statement.rs:391` and `binary_expression.rs:348` cast the parent to
  `RExpressionList`.

- `comments.rs` `place_leading_or_dangling_body_comment()` and `root.rs`
  `is_suppressed()` reach through `.expressions().first()`.

- `call_arguments.rs` `braced_expressions_is_groupable()` and
  `braced_expressions.rs`.

- LSP `handlers_format.rs` builds a synthetic `RRoot` from `r_expression_list()`
  and checks `R_EXPRESSION_LIST` in `find_deepest_enclosing_logical_lines()`.

Empty statements (`{;}`, `{1;;2}`) are a second trap. They are nodes, so
comments can attach to them, and Biome's debug checks panic on unformatted
comments or tokens. They must call `format_removed()` on the `;` and either
format or re-home their comments.

### Parser API differences

  | air                                                                         | aether                                                           |
  | ---                                                                         | ---                                                              |
  | `Parse`                                                                     | `RParse`                                                         |
  | `has_error()`                                                               | `has_errors()`                                                   |
  | `error()` / `into_error()` -> `ParseError` (implements `std::error::Error`) | `diagnostics()` / `into_diagnostics()` -> `Vec<ParseDiagnostic>` |
  | `into_result()`                                                             | gone                                                             |
  | `RParserOptions` derives serde                                              | no serde                                                         |
  | one whole-file error, no location                                           | one diagnostic per error, with ranges                            |

`workspace::format::FormatSourceError::Parse` wraps `ParseError` through
`#[from]`. That needs a new error type, because `ParseDiagnostic` doesn't
implement `std::error::Error`.

### Syntax crate differences

- `call_ext.rs` (`RCall::is_test_call()`) is not in aether. It's an inherent
  impl on what will be a foreign type, so it becomes a formatter-side function
  or extension trait.

- `OperatorPrecedence` is not in aether, and it is dead code in air today.
  Nothing outside `air_r_syntax` references it.

- `string_ext.rs` (`RStringValue::inner_string_text()`) is new in aether.

- Both pin the same `lionel-/biome` rev. This has to stay in lockstep forever:
  `biome_rowan` types in `aether_r_syntax` must be the same crate as the ones
  `air_r_formatter` uses. Bumping biome becomes a two-repo change.

### `xtask` today

`xtask/codegen` does two jobs:

1. Grammar codegen: `ast.rs`, `generate_*.rs`, `kind_src.rs`, `r_kinds_src.rs`,
   `language_kind.rs`. It reads `r.ungram` and writes `air_r_syntax` /
   `air_r_factory`. This is byte-for-byte the same as aether's `xtask/codegen`,
   except for crate names. It is deleted outright.

2. Formatter codegen: `formatter.rs`, 682 lines. It writes stubs for missing
   node formatters, fully regenerates the union formatters (`r/any/*.rs`), every
   `mod.rs`, and the 1646-line `generated.rs` (`AsFormat` / `IntoFormat` /
   `FormatRule` impls for every node). It deletes any file under `r/` that
   doesn't match a node. It uses `git2` to refuse to overwrite dirty files and
   to auto-stage its output.

The only thing formatter codegen needs from the grammar is `load_ast()`'s
classification of each type name: node, list (separated or not), bogus, or union
with variants. That function lives in the half being deleted, and reads
`r.ungram`, which moves to aether.

Formatter codegen has run 20 times in air's history, mostly in the first months.
The grammar changes rarely now.

### Options for formatter codegen after the move

A. Aether exposes the grammar model as a library (recommended). Move `r.ungram`,
`kind_src.rs`, `r_kinds_src.rs`, and the classification half of `ast.rs` out of
aether's `xtask/codegen` into a new `publish = false` crate, for example
`aether_r_grammar`, exposing `ast() -> AstSrc`. Aether's own codegen consumes
it. Air's `xtask` depends on it by git at the same rev as the other aether
crates, so the grammar it reads is always the grammar air compiles against. Air
keeps `formatter.rs` and loses everything else.

B. Read `r.ungram` from cargo's git checkout of aether through `cargo metadata`.
Works, but air must vendor `biome_ungrammar` and the classification code. Two
copies of the same code drift. Rejected.

C. Vendor a copy of `r.ungram` in air. Same drift problem. Rejected.

D. Drop formatter codegen. Replace `generated.rs` with a `macro_rules!` table,
write union formatters by hand, and add stubs by hand. The compiler catches most
drift, since a missing `AsFormat` impl or a non-exhaustive union `match` won't
compile. It deletes `xtask/codegen` entirely. The cost is losing a generator
Biome-style formatters all share, and every aether grammar change becoming hand
work.

E. Move formatter stub generation into aether, writing into an air checkout.
Couples aether to air's file layout. Rejected.

I recommend A, plus a `Mode::Verify` test in the style of rust-analyzer's
sourcegen tests: bumping the aether rev without re-running codegen fails CI.

Smaller cleanups worth making while in there:

- Drop the `LanguageKind` abstraction. There is one language.

- Drop `git2`. Stubs are only written when missing, and everything else it
  protects is fully generated. Aether already dropped it.

- Adopt aether's `project_root()`, which searches for `[workspace]` instead of
  counting ancestors.

- Possibly switch generated `r/any/mod.rs` to `r/any.rs` (question below).

One hazard for the swap: the generator deletes files it doesn't recognize.
`RBracedExpressions` -> `RBracedStatements` and `RExpressionList` ->
`RStatementList` must be `git mv`'d to their new paths *before* codegen runs, or
the hand-written bodies get deleted.

## The stack

Two repos. The aether PRs come first because air's swap depends on them.

### Aether

- A1. Extract `aether_r_grammar` (option A). Aether's codegen output is
  unchanged.

- A2. Audit air's `crates/air_r_parser/tests/snapshots/**` against aether's
  corpus, and port any construct that isn't covered. Air's fixtures are deleted
  in the swap, so aether's corpus has to hold everything first. Aether's retired
  oracle compared against air, so gaps should be small, but the audit makes
  sure.

- A3. Settle the repo URL and the rev or tag air pins.

### Air: prep PRs on `main`

Each one is green on its own, keeps the tree-sitter parser, and has independent
value. Order is by risk, smallest first.

- P1. Done. Fix the `binary_expression_sticky.R` fixture typo. Snapshot change only.

- P2. Done. Delete the dead `OperatorPrecedence`. Move `is_test_call()` into
  `air_r_formatter`.

- P3. Done. Switch the tree-sitter walker's whitespace attachment to Biome's JS rule
  (trailing on the token before). `deviations.md` asks for exactly this
  experiment: run the formatter suite before any formatter change, so a
  whitespace regression can't hide in the semicolon churn. Expected result: zero
  formatter snapshot diff. Parser snapshots and the LSP syntax snapshots will
  move. If a formatter snapshot moves, stop and fix aether first.

- P4. Done. Grammar additions that need no walker work: `AnyRForVariable`
  (`RIdentifier | RDots | RDotDotI | RBogusForVariable`), `RBogusSelector`,
  `RBogusParameterName`, `RBogusForVariable`, `ERROR_TOKEN`, plus `is_bogus()` /
  `to_bogus()` arms and generated formatter stubs. This fixes `for (... in x)`
  in air today: the walker already emits `R_DOTS` there. Add a spec test and a
  changelog bullet.

- P5. Done. Strings as one token. Walker emits
  `R_{SINGLE,DOUBLE}_QUOTED_STRING_LITERAL` / `R_RAW_STRING_LITERAL` instead of
  open/content/close (this reverts #497's shape). Rewrite `string_value.rs`.
  Follow the warning in `deviations.md` about `string_content.rs`: remove the
  trivia assert and switch `normalize_string_content_token()` to
  `text_trimmed()` in the same commit. `inner_string_text()` is not carried
  over, since nothing in air uses it. It arrives with the swap. Expected result: zero formatter snapshot diff, CRLF tests included.

- P6. Statement layer without semicolons. Grammar gains
  `RStatement { expression?, ';'? }`, `RStatementList`, and `RBracedStatements`.
  `RRoot.statements` replaces `expressions`. The walker wraps each child of the
  program and of `{}` in an `R_STATEMENT` and never fills the `;` slot. The
  formatter gains `FormatRStatement`, and every site in the `RStatement` section
  above is fixed, along with the LSP range formatter. Write `format_removed()`
  for the `;` slot here even though nothing produces it yet. Expected result:
  zero formatter snapshot diff. This is the riskiest formatter PR, and it lands
  with tree-sitter still underneath, so any snapshot movement can only come from
  the statement layer.

- P7. Parse API convergence. Rename to `RParse`, `has_errors()`,
  `diagnostics()`. Replace `ParseError` in `workspace` / CLI / LSP with whatever
  we decide below. The walker produces one `ParseDiagnostic` for the whole file,
  as now.

- P8. Optional `xtask` cleanup that doesn't depend on aether. Nothing here is
  needed for the move: the swap's only required `xtask` change is deleting
  grammar codegen and pointing `formatter.rs` at `aether_r_grammar`. Candidates:
  - A `Mode::Verify` test, so a stale `generated.rs` fails CI after an aether
    rev bump. This is the only one with real value.
  - Drop `git2`, which builds libgit2 (C) for every workspace build and clippy
    run.
  - Drop the single-language `LanguageKind` abstraction.
  - Adopt aether's `project_root()`.

### Air: the swap PR

Scoped to what can't move earlier. Checklist:

- [ ] Workspace `Cargo.toml`: add `aether_r_parser`, `aether_r_syntax`,
  `aether_r_factory` (and `aether_r_grammar` for `xtask`) as git deps pinned
  to one rev. Remove `air_r_*`, `biome_ungrammar`, `biome_unicode_table`,
  `tree-sitter`, `tree-sitter-r`.

- [ ] Delete `crates/air_r_parser`, `crates/air_r_syntax`,
  `crates/air_r_factory`, `crates/biome_ungrammar`, and
  `xtask/codegen/r.ungram`.

- [ ] Mechanical rename `air_r_syntax` -> `aether_r_syntax`, and the same for
  the factory and parser, as its own commit.

- [ ] `xtask`: delete grammar codegen and point `formatter.rs` at
  `aether_r_grammar::ast()`. Syntax crate ident becomes `aether_r_syntax`.
  Remove `just gen-grammar`. `git mv` renamed rule files before running
  `just gen-formatter`.

- [ ] Semicolons: `FormatRStatement` drops the token with `format_removed()`.
  Expression-less statements emit nothing and don't produce blank lines.
  Comments on empty statements get placement rules. New spec file
  `statements/semicolons.R` covering `1;2`, `1;`, `1;;2` in braces, `{;}`,
  `{;1}`, `1; # c`, `1 ; # c`, blank lines around a lone `;` in braces,
  `# fmt: skip` on a statement ending in `;`, and range formatting across
  `;`.

- [ ] New spec tests for things only aether can parse: `r"-|a|-"` raw strings
  and `%a\b%`.

- [ ] LSP: `Document` holds `RParse`. Range formatting builds
  `r_statement_list()` from the selected `RStatement`s directly. Remove the
  `TreeSitter` view kind and the matching VS Code command (question below).

- [ ] `air_formatter_test` / spec harness: only import changes, since
  `From<RParse> for AnyParse` exists.

- [ ] Snapshots: `crates/crates` crate list and LSP `document_syntax` snapshots.

- [ ] Docs: `CLAUDE.md` (crate layout, codegen), `CONTRIBUTING.md` (how to patch
  aether locally with `[patch."<aether url>"]`), `justfile`,
  `xtask/README.md`, `xtask/codegen/README.md`.

- [ ] `CHANGELOG.md`:
  - Files R rejects are now rejected: two expressions on one line, a stray
    top-level `;`, `pkg::` followed by a newline, `x$1`.
  - `r"-|...|-"` and `%a\b%` now parse.
  - Building air no longer needs a C compiler.
  - Performance, if the benchmark below shows a win.

- [ ] Validation beyond the spec suite:
  - Ecosystem diff: format a large corpus of real R code with `main` and with
    the branch, and diff the outputs. Every difference must trace to a
    deviation, and every file that newly fails to parse must be one R also
    rejects.
  - CLI benchmark on the same corpus.

### Follow-ups after the swap

- Print aether's diagnostics with locations from the CLI, instead of "Failed to
  parse due to syntax errors."

- Consider publishing parse diagnostics from the LSP, now that they carry
  ranges.

- Consider formatting around local errors (aether keeps the rest of the tree
  intact). Air refuses today, and that stays true in this plan.

- Downstream: Jarl should move from air's git crates to aether directly once air
  no longer exports `air_r_parser`.

## Decisions

- P3, P5, and P6 are in the stack. The throwaway walker work is worth getting
  provably neutral formatter PRs.

- `xtask` uses option A: aether exposes an `aether_r_grammar` crate, and air
  keeps only `formatter.rs` plus a verify test.

- `# fmt: skip` suppresses the whole `RStatement`, so a trailing `;` is kept
  verbatim. P6 moves directive and suppression checks from the expression onto
  the statement.

- Air pins `https://github.com/DavisVaughan/aether` by `rev`, like biome.
  Aether's `Cargo.toml` `repository` / `homepage` should be updated to match
  (part of A3).

- `feature/aether` only holds this plan. P1 to P8 target `main` one at a time,
  and the swap stacks on top at the end.

- The CLI keeps today's one-line parse error through P7 and the swap. Rich
  diagnostics are a follow-up.

- The swap drops the "view tree-sitter tree" LSP request and VS Code command,
  removing tree-sitter entirely.

- The ecosystem diff runs on a fixed list of GitHub repos cloned at pinned SHAs.
  The list goes in this plan before the swap starts.

## Open questions

1. Is P8 worth keeping? None of it is needed for the move. See the note under
   P8.
