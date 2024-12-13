# Generates the initial files for the formatter
gen-formatter:
  cargo run -p xtask_codegen -- formatter

# Generates the code of the grammar
gen-grammar:
    cargo run -p xtask_codegen -- grammar r

# Run the tests
test:
  cargo test

# Run the tests in verbose mode
# `--nocapture` to see our own `tracing` logs
# `--test-threads 1` to ensure `tracing` logs aren't interleaved
test-verbose:
  cargo test -- --nocapture --test-threads 1

# Run the quick formatter test
test-quick:
  cargo test --package air_r_formatter --test quick_test -- quick_test --exact --show-output --ignored

# Creates a new crate
new-crate name:
  cargo new --lib crates/{{snakecase(name)}}
  cargo run -p xtask_codegen -- new-crate --name={{snakecase(name)}}
