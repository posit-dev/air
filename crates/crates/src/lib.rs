// Generates `AIR_CRATE_NAMES`, a const array of the crate names in the air workspace,
// see `crates/build.rs`
include!(concat!(env!("OUT_DIR"), "/lib.rs"));

#[cfg(test)]
mod tests {
    use crate::AIR_CRATE_NAMES;

    #[test]
    fn test_crate_names() {
        insta::assert_debug_snapshot!(AIR_CRATE_NAMES);
    }
}
