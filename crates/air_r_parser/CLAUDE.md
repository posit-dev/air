# air_r_parser

A hand-written Rust parser for R, built on Biome's parser infrastructure.

## Invariants, do not break these

1. `parse(src).syntax().to_string() == src` for ALL inputs. Always.

2. Bogus nodes always come with at least one diagnostic.

3. The parser never panics and never loops forever.

## Test corpus

The corpus lives in `crates/air_r_parser_tests/tests/corpus/**/*.md`. One file
per concept, one test per `##` heading.

Each concept has two files. You write `X.md`, which holds headings, prose, and
\`\`\`r blocks:

````md
## Colon

```r
1:2
```

## Closing brace [error]

A stray `}` is reported and skipped, and the parse keeps going.

```r
}
```
````

`just test-snapshot-accept` generates `X.snap.md` from it. The snapshot repeats
the heading, the prose, and each source, and adds a cst and a diagnostics block
after each source. Never hand-edit a snapshot. To change a test, edit `X.md` and
re-run `just test-snapshot-accept`.

Prose between blocks is copied to the snapshot and otherwise ignored. Use it to
say why a test exists. Do not use an R comment for that, since a comment is
input and changes the tree.

The heading marker sets what the blocks under it must do:

- `[error]`: each block produces at least one error diagnostic.

- No marker: each block parses with zero diagnostics and no bogus nodes.

## Reference implementations

- Biome's JS/CSS parsers. When in doubt, do what `biome_js_parser` does.
