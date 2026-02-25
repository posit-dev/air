# air-formatter

An R formatter. Written in Rust.

This package provides the `air` command-line tool as a Python package, making it easy to install via `pip` or `uv`.

## Installation and usage

```bash
pip install air-formatter
```

Or with [uv](https://docs.astral.sh/uv/):

```bash
uv tool install air-formatter
```

After installation, the `air` CLI is available:

```bash
# Format R files
air format path/to/file.R

# Format all R files in a directory
air format path/to/directory/
```

You may also invoke it as a tool via [uv](https://docs.astral.sh/uv/) without installing it:

```bash
uvx --from air-formatter air format path/to/file.R
```

## About

Air is an opinionated R formatter built by [Posit](https://posit.co). For more information, see the [main repository](https://github.com/posit-dev/air).
