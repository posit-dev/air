use std::process::Command;
use std::process::Stdio;

use tempfile::TempDir;

use crate::helpers::CommandExt;
use crate::helpers::binary_path;
use crate::helpers::path_root;
use crate::helpers::relative_path_fixtures;

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

    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(path_root())
            .arg("format")
            .arg(path)
            .arg("--check")
            .run()
            .normalize_os_path_separator()
    );
}

#[test]
fn test_check_output_format() {
    let path1 = relative_path_fixtures().join("needs-formatting-1.R");
    let path1 = path1.to_str().unwrap();

    let path2 = relative_path_fixtures().join("needs-formatting-2.R");
    let path2 = path2.to_str().unwrap();

    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(path_root())
            .arg("format")
            .arg(path1)
            .arg(path2)
            .arg("--check")
            .run()
            .normalize_os_path_separator()
    );
}

#[test]
fn test_stdin_cant_supply_paths() -> anyhow::Result<()> {
    let directory = TempDir::new()?;
    let directory = directory.path();

    let test_path = "test.R";
    let test_contents = "1+\n1";
    std::fs::write(directory.join(test_path), test_contents)?;

    // Can't supply `format <path>` along with `--stdin-file-path`
    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg(test_path)
            .arg("--stdin-file-path")
            .arg(test_path)
            .arg("--no-color")
            .run_with_stdin(test_contents.to_string())
            .normalize_os_path_separator()
    );

    // Can't supply `format .` along with `--stdin-file-path`
    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg(".")
            .arg("--stdin-file-path")
            .arg(test_path)
            .arg("--no-color")
            .run_with_stdin(test_contents.to_string())
            .normalize_os_path_separator()
    );

    Ok(())
}

#[test]
fn test_stdin_uses_default_air_toml_settings() -> anyhow::Result<()> {
    let directory = TempDir::new()?;
    let directory = directory.path();

    let test_path = "test.R";
    let test_contents = "1+\n1";
    std::fs::write(directory.join(test_path), test_contents)?;

    let air_path = "air.toml";
    let air_contents = r#"
[format]
indent-width = 4
"#;
    std::fs::write(directory.join(air_path), air_contents)?;

    // Running in `directory` with a relative path to `test_path`. No `air.toml` found in
    // `directory` or its ancestors, so we use default settings.
    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg("--stdin-file-path")
            .arg(test_path)
            .run_with_stdin(test_contents.to_string())
            .remove_arguments()
    );

    Ok(())
}

#[test]
fn test_stdin_finds_air_toml_from_stdin_file_path() -> anyhow::Result<()> {
    // The directory we run from won't have an `air.toml` in it
    let current_directory = TempDir::new()?;
    let current_directory = current_directory.path();

    // The directory that `test.R` lives in will have one
    let directory = TempDir::new()?;
    let directory = directory.path();

    let test_path = "test.R";
    let test_contents = "1+\n1";
    std::fs::write(directory.join(test_path), test_contents)?;

    let air_path = "air.toml";
    let air_contents = r#"
[format]
indent-width = 4
"#;
    std::fs::write(directory.join(air_path), air_contents)?;

    // `current_directory` is where we run from, but we supply an absolute path of
    // `directory.join(test_path)`, and that is where we perform `air.toml` search from
    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(current_directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg("--stdin-file-path")
            .arg(directory.join(test_path))
            .run_with_stdin(test_contents.to_string())
            .remove_arguments()
    );

    Ok(())
}

#[test]
fn test_stdin_relative_stdin_file_paths_resolve_from_working_directory() -> anyhow::Result<()> {
    let directory = TempDir::new()?;
    let directory = directory.path();

    let test_path = "test.R";
    let test_contents = "1+\n1";
    std::fs::write(directory.join(test_path), test_contents)?;

    let air_path = "air.toml";
    let air_contents = r#"
[format]
indent-width = 4
"#;
    std::fs::write(directory.join(air_path), air_contents)?;

    // `directory` is supplied as current directory, and note that `test_path` is supplied
    // as a relative path. It is resolved relative to `directory` and then we find the
    // `air.toml` from there.
    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg("--stdin-file-path")
            .arg(test_path)
            .run_with_stdin(test_contents.to_string())
    );

    Ok(())
}

#[test]
fn test_stdin_fake_stdin_file_path() -> anyhow::Result<()> {
    let test_contents = "1+\n1";

    let directory = TempDir::new()?;
    let directory = directory.path();

    // Absolute path to `fake_file`
    let fake_file = directory.join("fake.R");

    // `fake_file` does not exist, but is still used as the place to start for `air.toml`
    // detection. With the current setup it doesn't find any `air.toml`.
    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(path_root())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg("--stdin-file-path")
            .arg(&fake_file)
            .run_with_stdin(test_contents.to_string())
            .remove_arguments()
    );

    let air_path = "air.toml";
    let air_contents = r#"
[format]
indent-width = 4
"#;
    std::fs::write(directory.join(air_path), air_contents)?;

    // Now we've written an `air.toml` in the `directory`. Even though `fake_file` itself
    // doesn't exist, we still find the `air.toml`.
    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(path_root())
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg("--stdin-file-path")
            .arg(&fake_file)
            .run_with_stdin(test_contents.to_string())
            .remove_arguments()
    );

    Ok(())
}

#[test]
fn test_stdin_empty_input() -> anyhow::Result<()> {
    let directory = TempDir::new()?;
    let directory = directory.path();

    let test_path = "test.R";

    // Empty stdin in write mode should succeed and produce empty stdout
    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg("--stdin-file-path")
            .arg(test_path)
            .run_with_stdin(String::new())
    );

    // Empty stdin in check mode should succeed (nothing to change)
    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg("--stdin-file-path")
            .arg(test_path)
            .arg("--check")
            .run_with_stdin(String::new())
    );

    Ok(())
}

#[test]
fn test_stdin_errors_on_parse_error() -> anyhow::Result<()> {
    let directory = TempDir::new()?;
    let directory = directory.path();

    let test_path = "test.R";
    let test_contents = "1+/1";
    std::fs::write(directory.join(test_path), test_contents)?;

    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg("--stdin-file-path")
            .arg(test_path)
            .arg("--no-color")
            .run_with_stdin(test_contents.to_string())
            .remove_arguments()
    );

    Ok(())
}

#[test]
fn test_stdin_works_correctly_with_check() -> anyhow::Result<()> {
    let directory = TempDir::new()?;
    let directory = directory.path();

    let test_path = "test.R";
    let test_contents = "1+1\n";
    std::fs::write(directory.join(test_path), test_contents)?;

    // This requires formatting, so errors with exit code 1. We don't "inform" the user
    // which file needs changes like we do when formatting paths, because it is obviously
    // stdin.
    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg("--stdin-file-path")
            .arg(test_path)
            .arg("--check")
            .run_with_stdin(test_contents.to_string())
            .remove_arguments()
    );

    let test_contents = "1 + 1\n";
    std::fs::write(directory.join(test_path), test_contents)?;

    // No changes required here (and we've carefully remembered the trailing newline!)
    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg("--stdin-file-path")
            .arg(test_path)
            .arg("--check")
            .run_with_stdin(test_contents.to_string())
            .remove_arguments()
    );

    Ok(())
}

/// TODO!: This test should FAIL once we change Air's defaults regarding directly supplied
/// files like `air format standalone-*.R` or `air format --stdin-file-path
/// standalone-*.R` and how they interact with `exclude`, `default-exclude`, and
/// `default-include`.
///
/// To better support pre-commit and RStudio, which will blindly provide whatever the user
/// has touched / saved to air as a file to format, we SHOULD respect the `exclude` rules
/// here by default and refuse to format this at the command line. If a user really wants
/// to bypass the exclude rules then they can do something like
/// `--ignore-exclude-for-directly-supplied-file` (or maybe we wouldn't allow this at
/// all?). This is honestly more in line with the LSP. If you do `Format Document` in
/// an excluded file, then we still refuse to format it!
///
/// Also, this would help with Quarto/Rmarkdown where people are trying to do `air format
/// test.Rmd` and they either get an obscure parse error or it actually fake works due to
/// chance. At the very least we'd now silently refuse to format this file because it
/// isn't in our `default-includes`, even though they provided it directly at the command
/// line. Or we could report a warning about this rather than being silent, and still
/// refuse to format.
#[test]
fn test_stdin_refuses_to_format_default_excludes() -> anyhow::Result<()> {
    let directory = TempDir::new()?;
    let directory = directory.path();

    let cpp11_path = "cpp11.R";
    let cpp11_contents = "1+1";
    std::fs::write(directory.join(cpp11_path), cpp11_contents)?;

    // `cpp11.R` is a `default-exclude` so it SHOULD refuse to format this
    // and just reemit the existing `1+1` asis
    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg("--stdin-file-path")
            .arg(cpp11_path)
            .run_with_stdin(cpp11_contents.to_string())
    );

    Ok(())
}

// TODO!: This should also refuse to format. See above.
#[test]
fn test_stdin_refuses_to_format_user_excludes() -> anyhow::Result<()> {
    let directory = TempDir::new()?;
    let directory = directory.path();

    let test_path = "test.R";
    let test_contents = "1+1";
    std::fs::write(directory.join(test_path), test_contents)?;

    let air_path = "air.toml";
    let air_contents = r#"
[format]
exclude = ["test.R"]
"#;
    std::fs::write(directory.join(air_path), air_contents)?;

    // `test.R` is an `exclude` so it SHOULD refuse to format this
    // and just reemit the existing `1+1` asis
    insta::assert_snapshot!(
        Command::new(binary_path())
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg("--stdin-file-path")
            .arg(test_path)
            .run_with_stdin(test_contents.to_string())
    );

    Ok(())
}
