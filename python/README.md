# air-formatter

An R formatter. Written in Rust.

This package provides the `air` command-line tool as a Python package, making it easy to install via `pip` or `uv`.

## Installation

Install a global installation of `air` with [uv](https://docs.astral.sh/uv/):

```bash
uv tool install air-formatter
```

or with pip

```bash
pip install air-formatter
```

This puts `air` on the PATH, so you can run:

```bash
# Format R file
air format path/to/file.R

# Format all R files in a directory
air format path/to/directory/
```

Alternatively, invoke air via `uvx` for one-off formatting without a global install:

```bash
uvx --from air-formatter air format path/to/file.R
```

To use a specific version of air:

```bash
# Global install
uv tool install air-formatter@0.8.2

# One off runs
uvx --from air-formatter@0.8.2 air format path/to/file.R
```

## About

Air is an opinionated R formatter built by [Posit](https://posit.co). For more information, see the [main repository](https://github.com/posit-dev/air).
