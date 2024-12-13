// Generates `AIR_CRATE_NAMES`, a const array of the crate names in the air workspace,
// see `lsp/src/build.rs`
include!(concat!(env!("OUT_DIR"), "/crates.rs"));
