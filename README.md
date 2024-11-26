- Install the Air CLI to `~/.cargo/bin` (which must be on your `PATH`):

  ```sh
  cargo install --path "crates/air_cli"
  ```

  Add a `--debug` flag if you're a developer.

- Install the VS Code extension:

  ```sh
  # The first time
  npm install --global vsce

  cd editors/code && rm -rf *.vsix && vsce package && positron --install-extension *.vsix
  ```

  Replace `positron` by `code` to install in VS Code. The CLI tools for Positron or VS Code need to be installed on your path.
