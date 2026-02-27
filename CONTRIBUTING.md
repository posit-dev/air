# Contributing to Air

Welcome! We really appreciate that you'd like to contribute to Air, thanks in advance!

# Release process

The release process of Air has some manual steps.

For each release of the CLI binary, we also create a release of:

-   The air-formatter PyPI package (with the same version number)

-   The VS Code and OpenVSX extension (with a different version number)

When you want to cut a release of Air:

-   Create a release branch

    -   Polish `CHANGELOG.md`

        -   Clean up any bullets that need reorganization

        -   Bump the `CHANGELOG.md` version

        -   Add a new `Development version` header (yep, right away - `cargo dist` is smart enough to ignore this header)

    -   Polish `editors/code/CHANGELOG.md`

        -   Mention that the new version of the binary is shipped with the extension

        -   Bump the `CHANGELOG.md` version

        -   Add a new `Development version` header

    -   In `crates/air/Cargo.toml`, bump the version.

        -   Run `cargo check` to sync `Cargo.lock`, in case your LSP didn't do it already.

    -   In `python/pyproject.toml`, bump the version.

    -   In `editors/code/package.json`, bump the minor version to the next even number for standard releases, or to the next odd number for preview releases.

    -   Open a PR with these changes.

-   Manually run the [release workflow](https://github.com/posit-dev/air/actions/workflows/release.yml)

    -   It runs on `workflow_dispatch`, and you must provide the `Release Tag` version to create. Always provide the same version that you used in `Cargo.toml`. Do not prefix the version with a `v`.

    -   The release workflow will:

        -   Build the Air binaries and installer scripts.

        -   Create and push a git tag for the version.

        -   Create a GitHub Release attached to that git tag.

        -   Attach the binaries and installer scripts to that GitHub Release as artifacts.

-   Manually run the [PyPI release workflow](https://github.com/posit-dev/air/actions/workflows/release-pypi.yml)

    -   It runs on `workflow_dispatch`, you must specify the version of the Air binary to release, it should match the version you provided above.

    -   The release workflow will:

        -   Build the Python wheels from the Air binaries.

        -   Push the Python wheels to PyPI if `!dry_run`.

-   Manually run the [extension release workflow](https://github.com/posit-dev/air/actions/workflows/release-vscode.yml)

    -   It runs on `workflow_dispatch`, and automatically pulls in the latest release binary of Air from the binary release workflow above. It will release to both the VS Code marketplace and the OpenVSX marketplace.

-   Bump the version of Air OpenVSX Extension version recorded in Positron's [`product.json`](https://github.com/posit-dev/positron/blob/main/product.json) and do a PR to Positron.

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

# Python wheels

Python wheel creation and publishing is handled automatically at release time through `release-pypi.yml`. Here we document parts of that process.

The Python wheels we distribute have the sole purpose of shipping the Air binary. There is no Python code in the wheel, and we don't support `python -m air` (meaning there is no `__main__.py` entry point). We expect it is more likely used as `uvx --from air-formatter air format .` (for a one off run) or as `uv tool install air-formatter` (for a global install of `air` which is symlinked into `~/.local/bin`), neither of which go through the thin Python shim that `python -m air` would do. Instead, these just call the shipped air binary directly.

The scaffolding for the Python package is in `python/`. We use `uv_build` as the build system, since it has nice support for the `scripts/` directory, which is where we put the Air binary for distribution.

In CI, `release-pypi.yml` collects the binaries from `release.yml` and builds a per-platform wheel that puts the platform specific binary into `scripts/`. In `pyproject.toml`, we've set `[tool.uv.build-backend.data]` so that `uv_build` knows to copy over `scripts/` into the resulting wheel at build time. We then run `uv build` to build a generic "any" wheel without a specific platform, however, because there is a platform specific binary in there we really need it to be tagged with a specific platform. So we have to retag it with the known platform tag as a follow up. These platform tags tell PyPI how to deliver the right wheel when the user requests `air-formatter`.

`release-pypi.yml` then collects the wheels and uses `uv publish` to send them off to PyPI. This is a specially named job! It uses PyPI's Trusted Publishing so that we don't need any tokens. Instead, on Davis's PyPI account we have told PyPI to expect that `posit-dev/air` has a `release-pypi.yml` workflow with a `environment: pypi` GitHub Environment set up, and when binaries are pushed from that source, PyPI will accept them without any additional tokens.

If you're testing the Python wheel generation locally, use `just build-wheel` to build the wheel, and `just run-wheel <air args>` to run it. This will build release Air, copy it into `scripts/`, build the "any" wheel (which is correct for you, since you just built Air), and then `run-wheel` will run it with `uv tool run`.

# VS Code Extension development installation

-   Build the development version of the Air CLI with:

    ``` bash
    cargo build
    ```

    This does not install the CLI, but instead builds it to `target/debug/air` (or `target/debug/air.exe` on Windows).

-   Install the development version of the VS Code or Positron extension:

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
