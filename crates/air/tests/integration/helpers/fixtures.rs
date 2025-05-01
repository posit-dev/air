use std::path::PathBuf;

/// Path to the crate root directory
pub fn path_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// Relative path to the air `fixtures/` directory
pub fn relative_path_fixtures() -> PathBuf {
    PathBuf::from("fixtures")
}
