// We allow dead_code to work around an issue where if any individual integration test
// doesn't use a particular exported function, it will get marked as unused (because each
// integration test is compiled separately, so it looks unused to that binary)
#[allow(dead_code)]
pub mod command;
#[allow(dead_code)]
pub mod fixtures;
