fn main() {
    // Rebuild when a new test file is added
    println!("cargo::rerun-if-changed=tests/corpus");
}
