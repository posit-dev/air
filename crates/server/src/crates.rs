// Generates `AIR_CRATE_NAMES`, a const array of the crate names in the air workspace,
// see `server/src/build.rs`
include!(concat!(env!("OUT_DIR"), "/crates.rs"));

#[cfg(test)]
mod tests {
    use crate::crates::AIR_CRATE_NAMES;

    #[test]
    fn test_crate_names() {
        insta::assert_debug_snapshot!(AIR_CRATE_NAMES);
    }
}
