use path_absolutize::Absolutize;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;

pub fn has_r_extension(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .map_or(false, is_r_extension)
}

pub fn is_r_extension(extension: &str) -> bool {
    matches!(extension, "r" | "R")
}

/// Convert any path to an absolute path (based on the current working
/// directory).
pub fn normalize_path<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref();
    if let Ok(path) = path.absolutize() {
        path.to_path_buf()
    } else {
        path.to_path_buf()
    }
}
