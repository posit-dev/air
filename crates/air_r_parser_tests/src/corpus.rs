use std::path::Path;
use std::path::PathBuf;

pub fn dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/corpus")
}

/// Every human-authored `*.md`, sorted
pub fn input_files() -> Vec<PathBuf> {
    files()
        .into_iter()
        .filter(|path| !is_snapshot(path))
        .collect()
}

/// Every generated `*.snap.md`, sorted
pub fn snapshot_files() -> Vec<PathBuf> {
    files()
        .into_iter()
        .filter(|path| is_snapshot(path))
        .collect()
}

/// The snapshot beside an input file
pub fn snapshot_of(input: &Path) -> PathBuf {
    input.with_extension("snap.md")
}

/// The path from the corpus root, which is what a snapshot banner records and
/// what a failure message should show
pub fn relative(path: &Path) -> String {
    let mut parts: Vec<String> = path
        .components()
        .rev()
        .take_while(|component| component.as_os_str() != "corpus")
        .map(|component| component.as_os_str().to_string_lossy().into_owned())
        .collect();

    parts.reverse();
    parts.join("/")
}

fn is_snapshot(path: &Path) -> bool {
    name(path).ends_with(".snap.md")
}

fn name(path: &Path) -> &str {
    path.file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
}

fn files() -> Vec<PathBuf> {
    let mut files = Vec::new();
    let mut directories = vec![dir()];

    while let Some(directory) = directories.pop() {
        let entries = std::fs::read_dir(&directory)
            .unwrap_or_else(|err| panic!("cannot read {}: {err}", directory.display()));

        for entry in entries {
            let path = entry.expect("cannot read directory entry").path();

            if path.is_dir() {
                directories.push(path);
            } else if name(&path).ends_with(".md") {
                files.push(path);
            }
        }
    }

    files.sort();
    files
}
