use std::process::Command;

use crate::helpers::binary_path;
use crate::helpers::path_fixtures;
use crate::helpers::path_root;
use crate::helpers::CommandExt;

#[test]
fn test_check_when_no_formatting_is_required() -> anyhow::Result<()> {
    let path = path_fixtures().join("formatted.R");
    let path = path.to_str().unwrap();

    insta::assert_snapshot!(Command::new(binary_path())
        .current_dir(path_root())
        .arg("format")
        .arg(path)
        .arg("--check")
        .run());

    Ok(())
}

#[test]
fn test_check_output_format() -> anyhow::Result<()> {
    let path1 = path_fixtures().join("needs-formatting-1.R");
    let path1 = path1.to_str().unwrap();

    let path2 = path_fixtures().join("needs-formatting-2.R");
    let path2 = path2.to_str().unwrap();

    insta::assert_snapshot!(Command::new(binary_path())
        .current_dir(path_root())
        .arg("format")
        .arg(path1)
        .arg(path2)
        .arg("--check")
        .run());

    Ok(())
}
