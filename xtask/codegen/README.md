# codegen

This crate contains local commands used to auto-generate source code.

## `just gen-grammar`

This command transforms the `*.ungram` files into the `air_*_syntax` and `air_*_factory` crates.

The project uses a fork of [`ungrammar`](https://github.com/rust-analyzer/ungrammar) to define the syntax of the language.

`ungrammar` uses a DSL to define and parse the grammar of a language.

Once the library parses the DSL files, some custom logic generates the AST APIs.

## `just gen-formatter`

This command creates a stub formatting rule in `air_r_formatter` for each node that doesn't have one yet, and regenerates `air_r_formatter/src/generated.rs`. Existing rules are never overwritten.

It stages the files it writes, and stops with an error if a file it would write has uncommitted changes.
