# Generates the initial files for the formatter
gen-formatter:
  cargo run -p xtask_codegen -- formatter

# Generates the code of the grammar
gen-grammar:
    cargo run -p xtask_codegen -- grammar r

# Run the parser and formatter tests
test:
  cargo test -p air_r_parser
  cargo test -p air_r_formatter

# Run the quick formatter test
quick:
  cargo test --package air_r_formatter --test quick_test -- quick_test --exact --show-output --ignored

# Creates a new crate
new-crate name:
  cargo new --lib crates/{{snakecase(name)}}
  cargo run -p xtask_codegen -- new-crate --name={{snakecase(name)}}
