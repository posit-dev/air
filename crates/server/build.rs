use std::env;
use std::fs;
use std::path::Path;

extern crate cargo_metadata;

fn main() {
    write_workspace_crate_names();
}

/// Write out a constant array of air crate names as `AIR_CRATE_NAMES` at build time
fn write_workspace_crate_names() {
    let dir = env::var_os("OUT_DIR").unwrap();
    let path = Path::new(&dir).join("crates.rs");

    // Equivalent to `cargo metadata --no-deps`
    let mut cmd = cargo_metadata::MetadataCommand::new();
    cmd.no_deps();
    let metadata = cmd.exec().unwrap();

    let mut packages: Vec<String> = metadata
        .workspace_packages()
        .into_iter()
        .map(|package| package.name.clone())
        .collect();

    // Sort for stability across `cargo metadata` versions
    packages.sort();

    let packages: Vec<String> = packages
        .into_iter()
        .map(|package| String::from("\"") + package.as_str() + "\",")
        .collect();

    let packages = packages.join(" ");

    let contents = format!("pub(crate) const AIR_CRATE_NAMES: &[&str] = &[{packages}];");

    fs::write(&path, contents).unwrap();
    println!("cargo::rerun-if-changed=build.rs");
}
