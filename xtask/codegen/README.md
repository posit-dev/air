# codegen

This crate contains local commands used to auto-generate source code.

## `cargo codegen grammar`

This command transforms the `*.ungram` files into the `air_*_syntax` and `air_*_factory` crates.

The project uses a fork of [`ungrammar`](https://github.com/rust-analyzer/ungrammar) to define the syntax of the language.

`ungrammar` uses a DSL to define and parse the grammar of a language.

Once the library parses the DSL files, some custom logic generates the AST APIs.
