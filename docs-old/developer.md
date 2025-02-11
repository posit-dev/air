# Release

The release process of the air cli has some manual steps. One complication is that for each release of the CLI binary, we create a new release of the extension as this is our primary way of distributing Air. The version numbers between the CLI binary and the extension are not required to be the same.

When you want to cut a release:

- Create a release branch

    - Polish `CHANGELOG.md`, bump the version and add a new `Development version` header (yep, right away - `cargo dist` is smart enough to ignore this header).

    - Polish `editors/code/CHANGELOG.md`, bump the version and add a new `Development version` header.

    - In `crates/air/Cargo.toml`, bump the version.

    - Run `cargo check` to sync `Cargo.lock`.

    - In `editors/code/package.json`, bump the minor version to the next even number for standard releases, or to the next odd number for preview releases.

    - Open a PR with these changes.

- Manually run the [release workflow](https://github.com/posit-dev/air/actions/workflows/release.yml)

    - It runs on `workflow_dispatch`, and you must provide the `Release Tag` version to create. Always provide the same version that you used in `Cargo.toml`. Do not prefix the version with a `v`.

    - The release workflow will:

        - Build the binaries and installer scripts.

        - Create and push a git tag for the version.

        - Create a GitHub Release attached to that git tag.

        - Attach the binaries and scripts to that GitHub Release as artifacts.

- Manually run the [extension release workflow](https://github.com/posit-dev/air/actions/workflows/release-vscode.yml)

    - It runs on `workflow_dispatch`, and automatically pulls in the latest release binary of Air from the binary release workflow above.

- Merge the release branch

    - There is no need to bump to an intermediate "dev version" after a release.

# Development installation

Install the dev version of the Air cli with:

```sh
cargo install --path crates/air --debug
```

This installs it to `~/.cargo/bin` (which must be on your `PATH`), and can be removed with `cargo uninstall air`.

Install the dev version of the VS Code extension:

```sh
# Install for Positron
(cd editors/code && (rm -rf *.vsix || true) && npx @vscode/vsce package && positron --install-extension *.vsix)

# Install for VS Code
(cd editors/code && (rm -rf *.vsix || true) && npx @vscode/vsce package && code --install-extension *.vsix)
```

The CLI tools for Positron or VS Code need to be installed on your path using the command palette command `Shell Command: Install 'code'/'positron' command in PATH`.

# Testing

We use [nextest](https://nexte.st/) for testing rather than a standard `cargo test`, primarily because nextest runs each test in its own process rather than in its own thread.
This is critical for us, as Air has global objects that can only be set up once per process (such as the global logger).
Additionally, using one process per test means that it is impossible for one test to interfere with another (so you don't have to worry about test cleanup).
Tests are still run in parallel, using multiple processes, and this ends up being quite fast and reliable.

Install the nextest cli tool using a [prebuilt binary](https://nexte.st/docs/installation/pre-built-binaries/).

Run tests locally with `just test`, which calls `cargo nextest run`.
Run insta snapshot tests in "update" mode with `just test-insta`.

On CI we use the nextest profile found in `.config/nextest.toml`.
