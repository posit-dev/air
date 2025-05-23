use std::process::Command;

use tempfile::TempDir;

use crate::helpers::binary_path;
use crate::helpers::path_root;
use crate::helpers::relative_path_fixtures;
use crate::helpers::CommandExt;

#[test]
fn test_default_options() -> anyhow::Result<()> {
    let directory = TempDir::new()?;
    let directory = directory.path();

    let path = "test.R";
    std::fs::write(directory.join(path), "1 + 1")?;

    let output = Command::new(binary_path())
        .current_dir(directory)
        .arg("format")
        .arg(path)
        .run();

    assert!(output.status.success());
    Ok(())
}

#[test]
fn test_default_exclude_patterns() -> anyhow::Result<()> {
    let directory = TempDir::new()?;
    let directory = directory.path();

    let test_path = "test.R";
    let test_contents = "1+1";
    std::fs::write(directory.join(test_path), test_contents)?;

    let cpp11_path = "cpp11.R";
    let cpp11_contents = "1+1";
    std::fs::write(directory.join(cpp11_path), cpp11_contents)?;

    let output = Command::new(binary_path())
        .current_dir(directory)
        .arg("format")
        .arg(".")
        .run();

    assert!(output.status.success());

    // Only `test.R` should be formatted, `cpp11.R` is a default exclude
    assert!(test_contents != std::fs::read_to_string(directory.join(test_path))?);
    assert_eq!(
        cpp11_contents,
        std::fs::read_to_string(directory.join(cpp11_path))?
    );

    Ok(())
}

#[test]
fn test_default_exclude_patterns_with_explicit_format_file_request() -> anyhow::Result<()> {
    let directory = TempDir::new()?;
    let directory = directory.path();

    let cpp11_path = "cpp11.R";
    let cpp11_contents = "1+1";
    std::fs::write(directory.join(cpp11_path), cpp11_contents)?;

    // Formatting on `cpp11.R` is explicitly requested, since this does not require
    // any file discovery, we format it even though it matches a default `exclude`.
    let output = Command::new(binary_path())
        .current_dir(directory)
        .arg("format")
        .arg(cpp11_path)
        .run();

    assert!(output.status.success());

    assert_eq!(
        std::fs::read_to_string(directory.join(cpp11_path))?,
        String::from("1 + 1\n")
    );

    Ok(())
}

#[test]
fn test_default_exclude_patterns_with_explicit_format_folder_request() -> anyhow::Result<()> {
    let directory = TempDir::new()?;
    let directory = directory.path();

    let renv_directory = "renv";
    std::fs::create_dir(directory.join(renv_directory))?;

    let activate_path = "activate.R";
    let activate_contents = "1+1";
    std::fs::write(
        directory.join(renv_directory).join(activate_path),
        activate_contents,
    )?;

    // Formatting on `air format renv` is explicitly requested. We don't auto
    // accept folders provided on the command line, so this goes through the standard
    // path and ends up excluding everything in `renv/`.
    let output = Command::new(binary_path())
        .current_dir(directory)
        .arg("format")
        .arg(renv_directory)
        .run();

    assert!(output.status.success());

    assert_eq!(
        activate_contents,
        std::fs::read_to_string(directory.join(renv_directory).join(activate_path))?
    );

    Ok(())
}

#[test]
fn test_modified_exclude_patterns() -> anyhow::Result<()> {
    let directory = TempDir::new()?;
    let directory = directory.path();

    let test_path = "test.R";
    let test_contents = "1+1";

    let cpp11_path = "cpp11.R";
    let cpp11_contents = "1+1";

    let air_path = "air.toml";
    let air_contents = r#"
[format]
exclude = ["test.R"]
default-exclude = false
"#;

    // Turn off `default-exclude`, turn on the custom `exclude`
    std::fs::write(directory.join(test_path), test_contents)?;
    std::fs::write(directory.join(cpp11_path), cpp11_contents)?;
    std::fs::write(directory.join(air_path), air_contents)?;

    // Only `cpp11.R` should be formatted
    let output = Command::new(binary_path())
        .current_dir(directory)
        .arg("format")
        .arg(".")
        .run();

    assert!(output.status.success());

    assert_eq!(
        test_contents,
        std::fs::read_to_string(directory.join(test_path))?
    );
    assert!(cpp11_contents != std::fs::read_to_string(directory.join(cpp11_path))?);

    Ok(())
}

#[test]
fn test_check_returns_cleanly_for_multiline_strings_with_crlf_line_endings() {
    let path = relative_path_fixtures()
        .join("crlf")
        .join("multiline_string_value.R");

    let output = Command::new(binary_path())
        .current_dir(path_root())
        .arg("format")
        .arg(path)
        .arg("--check")
        .run();

    assert!(output.status.success());
}

#[test]
fn test_check_when_no_formatting_is_required() {
    let path = relative_path_fixtures().join("formatted.R");
    let path = path.to_str().unwrap();

    insta::assert_snapshot!(Command::new(binary_path())
        .current_dir(path_root())
        .arg("format")
        .arg(path)
        .arg("--check")
        .run()
        .normalize_os_path_separator());
}

#[test]
fn test_check_output_format() {
    let path1 = relative_path_fixtures().join("needs-formatting-1.R");
    let path1 = path1.to_str().unwrap();

    let path2 = relative_path_fixtures().join("needs-formatting-2.R");
    let path2 = path2.to_str().unwrap();

    insta::assert_snapshot!(Command::new(binary_path())
        .current_dir(path_root())
        .arg("format")
        .arg(path1)
        .arg(path2)
        .arg("--check")
        .run()
        .normalize_os_path_separator());
}
