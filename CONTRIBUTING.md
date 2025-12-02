# Contributing to Air

Welcome! We really appreciate that you'd like to contribute to Air, thanks in advance!

# Release process

The release process of Air has some manual steps. One complication is that for each release of the CLI binary, we create a new release of the VS Code / OpenVSX extension as this is our primary way of distributing Air. The version numbers between the CLI binary and the VS Code / OpenVSX extension will end up being different.

When you want to cut a release of the Air binary and Air VS Code / OpenVSX extension:

-   Create a release branch

    -   Polish `CHANGELOG.md`, bump the version and add a new `Development version` header (yep, right away - `cargo dist` is smart enough to ignore this header).

    -   Polish `editors/code/CHANGELOG.md`, bump the version and add a new `Development version` header.

        -   Mention that the new version of the binary is shipped with the extension.

    -   In `crates/air/Cargo.toml`, bump the version.

    -   Run `cargo check` to sync `Cargo.lock`, in case your LSP didn't do it already.

    -   In `editors/code/package.json`, bump the minor version to the next even number for standard releases, or to the next odd number for preview releases.

    -   Open a PR with these changes.

-   Manually run the [release workflow](https://github.com/posit-dev/air/actions/workflows/release.yml)

    -   It runs on `workflow_dispatch`, and you must provide the `Release Tag` version to create. Always provide the same version that you used in `Cargo.toml`. Do not prefix the version with a `v`.

    -   The release workflow will:

        -   Build the binaries and installer scripts.

        -   Create and push a git tag for the version.

        -   Create a GitHub Release attached to that git tag.

        -   Attach the binaries and scripts to that GitHub Release as artifacts.

-   Manually run the [extension release workflow](https://github.com/posit-dev/air/actions/workflows/release-vscode.yml)

    -   It runs on `workflow_dispatch`, and automatically pulls in the latest release binary of Air from the binary release workflow above. It will release to both the VS Code marketplace and the OpenVSX marketplace.

-   Bump the version of Air recorded in Positron's [`product.json`](https://github.com/posit-dev/positron/blob/main/product.json).

-   Merge the release branch

    -   There is no need to bump to an intermediate "dev version" after a release.

# Zed extension release process

It is rare to need to do a Zed extension release because that code is fairly static. It knows how to download the latest version of Air, so we only need to change something there if we alter the way the extension itself works.

For a new release:

-   Create a release branch in Air called `release-zed/x.y.z`

    -   Update the `version` in `editors/zed/Cargo.toml`.

    -   Update the `version` in `editors/zed/extension.toml` to the same version.

    -   Open a PR with these changes, and go ahead and squash merge the PR. Note the commit hash of the merged commit.

-   Fork the [`zed-industries/extensions`](https://github.com/zed-industries/extensions) repository if you haven't yet.

-   Create a release branch in `zed-industries/extensions` called `update-air/x.y.z`

    -   Update the Air submodule in `extensions/` to point to the commit of the newly merged PR from the Air Zed extension release. This looks something like:

        ``` bash
        # Move into the submodule so `git` commands work on the submodule
        cd extensions/air
        # Fetch latest changes to Air
        git fetch origin
        # Checkout the Air sha you want to pin Zed to
        git checkout <commit-sha>
        # Move back to top level
        cd ../..
        # Commit the Air sha update
        git add extensions/air
        ```

    -   Update the Air `version` in `extensions.toml`. Double check that this `version` matches the `version` set in the Air Zed extension.

    -   Do a PR to `zed-industries/extensions` with these changes.

If you have any questions about the process, refer to [Zed's update guide](https://zed.dev/docs/extensions/developing-extensions#updating-an-extension).

# VS Code Extension development installation

-   Build the development version of the Air CLI with:

    ``` bash
    cargo build
    ```

    This does not install the CLI, but instead builds it to `target/debug/air` (or `target/debug/air.exe` on Windows).

-   Install/activate the coordinated version of Node:

    ```sh
    (cd editors/code && nvm use)
    ```

-   Install the development version of the VS Code or Positron extension.
    From the root:

    ``` bash
    # Install for Positron
    (cd editors/code && (rm -rf *.vsix || true) && npx @vscode/vsce package && positron --install-extension *.vsix)

    # Install for VS Code
    (cd editors/code && (rm -rf *.vsix || true) && npx @vscode/vsce package && code --install-extension *.vsix)
    ```

    The CLI tools for Positron or VS Code need to be installed on your path using the command palette command `Shell Command: Install 'code'/'positron' command in PATH`.

-   In your `settings.json`, set `air.executablePath` to the full path to the debug Air CLI binary you created in step one. Then set `air.executableStrategy` to `"path"` and restart the extension. At this point you can now swap between `"path"` and `"bundled"` to turn the debug binary on and off.

# Zed Extension development installation

Zed has a great guide on [developing extensions](https://zed.dev/docs/extensions/developing-extensions) if you are working on the development version of the Air Zed extension. Read that first.

To install the development version of the Air extension:

-   Run `zed: install dev extension`

-   Select the `editors/zed` directory. This will install the development version of the Air extension. An `extension.wasm` file should be created in the `editors/zed` directory. This file is `.gitignore`d.

To rebuild the extension, Zed has a shortcut process:

-   Run `zed: extensions`

-   Find `Air` in the list

-   Click `Rebuild`

# Testing

We use [nextest](https://nexte.st/) for testing rather than a standard `cargo test`, primarily because nextest runs each test in its own process rather than in its own thread. This is critical for us, as Air has global objects that can only be set up once per process (such as the global logger). Additionally, using one process per test means that it is impossible for one test to interfere with another (so you don't have to worry about test cleanup). Tests are still run in parallel, using multiple processes, and this ends up being quite fast and reliable.

Install the nextest cli tool using a [prebuilt binary](https://nexte.st/docs/installation/pre-built-binaries/).

Run tests locally with `just test`, which calls `cargo nextest run`. Run insta snapshot tests in "update" mode with `just test-insta`.

On CI we use the nextest profile found in `.config/nextest.toml`.
