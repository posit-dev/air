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

# Creates a new crate
new-crate name:
  cargo new --lib crates/{{snakecase(name)}}
  cargo run -p xtask_codegen -- new-crate --name={{snakecase(name)}}
