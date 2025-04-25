use std::path::Path;
use std::path::PathBuf;

pub fn path_fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("fixtures")
}
