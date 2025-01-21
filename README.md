Air <img src="docs/images/air.png" align="right" height=160 />
============================================================================

[![Actions status](https://github.com/posit-dev/air/actions/workflows/test.yml/badge.svg)](https://github.com/posit-dev/air/actions)

> [!NOTE]
> air is currently in alpha. Expect breaking changes both in the API and in formatting results.

An R formatter and language server, written in Rust.

# Installation

Install air using our standalone installers.

On macOS and Linux:

```shell
curl -LsSf https://github.com/posit-dev/air/releases/latest/download/air-installer.sh | sh
```

On Windows:

```shell
powershell -c "irm https://github.com/posit-dev/air/releases/latest/download/air-installer.ps1 | iex"
```

For a specific version:

```shell
curl -LsSf https://github.com/posit-dev/air/releases/download/0.1.1/air-installer.sh | sh
powershell -c "irm https://github.com/posit-dev/air/releases/download/0.1.1/air-installer.ps1 | iex"
```

The installer scripts will automatically add air to your `PATH`. The very first time you install air, you'll need to restart your shell for the `PATH` modifications to be applied.

# Acknowledgements and inspiration

air draws inspiration from many sources including [roslyn](https://github.com/dotnet/roslyn), [swift](https://github.com/swiftlang/swift), [rust-analyzer](https://github.com/rust-lang/rust-analyzer), [prettier](https://github.com/prettier/prettier), [biome](https://github.com/biomejs/biome), and [ruff](https://github.com/astral-sh/ruff). These are all excellent tools that provide either formatters, language servers, or both, all of which have influenced design decisions in air.

We are particularly thankful to [biome](https://github.com/biomejs/biome), as air is built on top of their language agnostic tooling for both building a [rowan](https://github.com/rust-analyzer/rowan) syntax tree and implementing a formatter. Biome is an open source project maintained by community members, please consider [sponsoring them](https://github.com/sponsors/biomejs#sponsors).

# Developer notes

Install the dev version of the air cli with:

```sh
cargo install --path crates/air --debug
```

This installs it to `~/.cargo/bin` (which must be on your `PATH`), and can be removed with `cargo uninstall air`.

Install the dev version of the VS Code extension:

```sh
# The first time
npm install --global vsce

# Install for Positron
cd editors/code && rm -rf *.vsix && vsce package && positron --install-extension *.vsix

# Install for VS Code
cd editors/code && rm -rf *.vsix && vsce package && code --install-extension *.vsix
```

The CLI tools for Positron or VS Code need to be installed on your path using the command palette command `Shell Command: Install 'code'/'positron' command in PATH`.
