# CLI

The release process of the air cli has some manual steps. When you want to cut a release:

- Create a release branch

    - Polish `CHANGELOG.md`, bump the version and add a new `Development version` header.

    - In `crates/air/Cargo.toml`, bump the version.

    - Run `cargo check` to sync `Cargo.lock`.

    - Do a PR and merge the release branch.

- Manually run the [release workflow](https://github.com/posit-dev/air/actions/workflows/release.yml)

    - It runs on `workflow_dispatch`, and you must provide the `Release Tag` version to create. Always provide the same version that you used in `Cargo.toml`. Do not prefix the version with a `v`.

    - The release workflow will:

        - Build the binaries and installer scripts.

        - Create and push a git tag for the version.

        - Create a GitHub Release attached to that git tag.

        - Attach the binaries and scripts to that GitHub Release as artifacts.

There is no need to bump to an intermediate "dev version" after a release.
