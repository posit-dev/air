use std::process::Command;

use crate::helpers::binary_path;
use crate::helpers::CommandExt;

#[test]
fn test_help() {
    insta::assert_snapshot!(&mut Command::new(binary_path()).run());
    insta::assert_snapshot!(Command::new(binary_path()).arg("help").run());
    insta::assert_snapshot!(Command::new(binary_path()).arg("--help").run());
    insta::assert_snapshot!(Command::new(binary_path()).arg("-h").run());
}

#[test]
fn test_format_help() {
    insta::assert_snapshot!(Command::new(binary_path()).arg("format").run());
    insta::assert_snapshot!(Command::new(binary_path())
        .arg("format")
        .arg("--help")
        .run());
    insta::assert_snapshot!(Command::new(binary_path()).arg("format").arg("-h").run());
}
