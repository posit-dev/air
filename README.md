# Air <img src="docs/images/air.png" align="right" height="160"/>

[![Actions status](https://github.com/posit-dev/air/actions/workflows/test.yml/badge.svg)](https://github.com/posit-dev/air/actions)

An R formatter and language server, written in Rust.

# Installation

Air is usable both as a command line tool and as a language server inside your favorite code editors. If you'd like to use Air within a code editor, we recommend reading our [editors guide](https://posit-dev.github.io/air/editors.html). If you'd just like to use Air from the command line, you can install Air using our standalone installers.

On macOS and Linux:

``` bash
curl -LsSf https://github.com/posit-dev/air/releases/latest/download/air-installer.sh | sh
```

On Windows:

``` powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/posit-dev/air/releases/latest/download/air-installer.ps1 | iex"
```

For a specific version:

``` bash
curl -LsSf https://github.com/posit-dev/air/releases/download/0.9.0/air-installer.sh | sh
```

``` powershell
powershell -ExecutionPolicy Bypass -c "irm https://github.com/posit-dev/air/releases/download/0.9.0/air-installer.ps1 | iex"
```

The installer scripts will automatically add Air to your `PATH`.
The very first time you install Air, for the `PATH` modifications to be applied you'll need to restart your terminal.

## uv

If you use [uv](https://docs.astral.sh/uv/), you can install Air via the [air-formatter](https://pypi.org/project/air-formatter/) package:

``` bash
# Global install
uv tool install air-formatter
air format path/to/my/script.R

# Invoke once without installing
uvx --from air-formatter air format path/to/my/script.R
```

## Homebrew

If you're on macOS and are familiar with [Homebrew](https://brew.sh/), you can install the [air formula](https://formulae.brew.sh/formula/air) with:

``` bash
brew install air
```

Homebrew support is community maintained.

## Pixi

You can use [Pixi](https://pixi.prefix.dev/latest/) to install Air because it is on [conda-forge](https://github.com/conda-forge/air-feedstock):

``` bash
# Add to a project
pixi add air
pixi run air format path/to/my/script.R

# Install globally
pixi global install air

# Temporary environment
pixi exec air format path/to/my/script.R
```

Pixi support is community maintained.

## mise

You can use [mise](https://mise.jdx.dev/) to install Air via [conda-forge](https://github.com/conda-forge/air-feedstock):

``` bash
# Add to a project
mise use conda:air

# Install globally
mise use --global conda:air
```

mise support is community maintained.

# Acknowledgements and inspiration

First and foremost, Air would not exist without the preexisting work and dedication poured into [styler](https://github.com/r-lib/styler). Created by [Lorenz Walthert](https://github.com/lorenzwalthert) and [Kirill Müller](https://github.com/krlmlr), styler proved that the R community does care about how their code is formatted, and had been the primary implementation of the [tidyverse style guide](https://style.tidyverse.org/) for many years.

Additionally, Air draws inspiration from many non-R sources including [rust-analyzer](https://github.com/rust-lang/rust-analyzer), [prettier](https://github.com/prettier/prettier), [biome](https://github.com/biomejs/biome), and [ruff](https://github.com/astral-sh/ruff). These are all excellent tools that provide either formatters, language servers, or both, all of which have influenced design decisions in Air, produced libraries on which we depend, or wrote code that we included in the project.

We are particularly thankful to [biome](https://github.com/biomejs/biome), as Air is built on top of their language agnostic tooling for both building a [rowan](https://github.com/rust-analyzer/rowan) syntax tree and implementing a formatter. Biome is an open source project maintained by community members, please consider [sponsoring them](https://github.com/sponsors/biomejs#sponsors).
