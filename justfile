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
test:
  cargo nextest run

# Run the tests in verbose mode
# `--no-capture` forces stdout/stderr to be shown for all tests, not just failing ones,
# and also forces them to be run sequentially so you don't see interleaved live output
test-verbose:
  cargo nextest run --no-capture

# Run the insta tests in update mode
test-insta:
  cargo insta test --test-runner nextest

# Run the quick formatter test
test-quick:
  cargo test --package air_r_formatter --test quick_test -- quick_test --exact --show-output --ignored

# Creates a new crate
new-crate name:
  cargo new --lib crates/{{snakecase(name)}}
  cargo run -p xtask_codegen -- new-crate --name={{snakecase(name)}}
