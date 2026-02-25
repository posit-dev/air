# Generates the initial files for the formatter
gen-formatter:
  cargo run -p xtask_codegen -- formatter

# Generates the code of the grammar
gen-grammar:
    cargo run -p xtask_codegen -- grammar r

# Generates the `air.schema.json`
gen-schema:
    cargo run -p xtask_codegen -- json-schema

# Run the tests
test *ARGS:
  cargo nextest run {{ARGS}}

# Run the tests in verbose mode
# `--no-capture` forces stdout/stderr to be shown for all tests, not just failing ones,
# and also forces them to be run sequentially so you don't see interleaved live output
test-verbose *ARGS:
  cargo nextest run --no-capture {{ARGS}}

# Run the insta tests in update mode
test-insta *ARGS:
  cargo insta test --test-runner nextest {{ARGS}}

# Run the quick formatter test
test-quick:
  cargo test --package air_r_formatter --test quick_test -- quick_test --exact --show-output --ignored

install-vscode:
  cd editors/code && rm -rf *.vsix && vsce package && code --install-extension *.vsix

install-positron:
  cd editors/code && rm -rf *.vsix && vsce package && positron --install-extension *.vsix

# For local wheel testing. Will generate:
#
# ```
# wheel/air_formatter-{version}-py3-none-any.whl
# ```
#
# which works on the system it was built on.
# Automatically installs it with `uv tool install`.
install-wheel:
  cargo build --release
  mkdir -p python/scripts
  cp target/release/air python/scripts/air
  uv build --wheel --out-dir wheel/ python/
  uv tool install wheel/*.whl
