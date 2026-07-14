use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::Stdio;

use tempfile::TempDir;
use workspace::config::user_config_directory_env_var;

use crate::helpers::CommandExt;
use crate::helpers::binary_path;

fn write_user_air_toml(config: &Path, contents: &str) -> anyhow::Result<()> {
    let directory = config.join("air");
    std::fs::create_dir(&directory)?;
    std::fs::write(directory.join("air.toml"), contents)?;
    Ok(())
}

fn user_air_toml(config: &Path) -> PathBuf {
    config.join("air").join("air.toml")
}

#[test]
fn test_user_config_applies_when_no_project_air_toml() -> anyhow::Result<()> {
    let user_config_directory = TempDir::new()?;
    let user_config_directory = user_config_directory.path();
    write_user_air_toml(user_config_directory, "[format]\nindent-width = 8\n")?;

    let directory = TempDir::new()?;
    let directory = directory.path();

    let test_path = "test.R";
    std::fs::write(directory.join(test_path), "if (TRUE) {\n1\n}\n")?;

    // No project `air.toml`, so the user level `indent-width = 8` applies
    let output = Command::new(binary_path())
        .env(user_config_directory_env_var(), user_config_directory)
        .current_dir(directory)
        .arg("format")
        .arg(test_path)
        .run();

    assert!(output.status.success());
    assert_eq!(
        std::fs::read_to_string(directory.join(test_path))?,
        "if (TRUE) {\n        1\n}\n"
    );

    Ok(())
}

#[test]
fn test_user_config_applies_to_stdin() -> anyhow::Result<()> {
    let user_config_directory = TempDir::new()?;
    let user_config_directory = user_config_directory.path();
    write_user_air_toml(user_config_directory, "[format]\nindent-width = 8\n")?;

    let directory = TempDir::new()?;
    let directory = directory.path();

    // The user level `indent-width = 8` applies to `--stdin-file-path` as well,
    // and shows up in the snapshot
    insta::assert_snapshot!(
        Command::new(binary_path())
            .env(user_config_directory_env_var(), user_config_directory)
            .current_dir(directory)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .arg("format")
            .arg("--stdin-file-path")
            .arg("test.R")
            .run_with_stdin("if (TRUE) {\n1\n}\n".to_string())
            .remove_arguments()
    );

    Ok(())
}

#[test]
fn test_project_air_toml_wins_over_user_config() -> anyhow::Result<()> {
    let user_config_directory = TempDir::new()?;
    let user_config_directory = user_config_directory.path();
    write_user_air_toml(
        user_config_directory,
        "[format]\nindent-width = 8\nindent-style = \"tab\"\n",
    )?;

    let directory = TempDir::new()?;
    let directory = directory.path();

    // A project `air.toml` fully shadows the user level one
    std::fs::write(directory.join("air.toml"), "[format]\nindent-width = 4\n")?;

    let test_path = "test.R";
    std::fs::write(directory.join(test_path), "if (TRUE) {\n1\n}\n")?;

    let output = Command::new(binary_path())
        .env(user_config_directory_env_var(), user_config_directory)
        .current_dir(directory)
        .arg("format")
        .arg(test_path)
        .run();

    assert!(output.status.success());

    // Formatted with the project's `indent-width = 4`, not the user level `8`, and with
    // the project implied default of spaces, not the user level tabs.
    assert_eq!(
        std::fs::read_to_string(directory.join(test_path))?,
        "if (TRUE) {\n    1\n}\n"
    );

    Ok(())
}

#[test]
fn test_user_config_with_dot_air_toml() -> anyhow::Result<()> {
    let user_config_directory = TempDir::new()?;
    let user_config_directory = user_config_directory.path();

    let user_directory = user_config_directory.join("air");
    std::fs::create_dir(&user_directory)?;

    let directory = TempDir::new()?;
    let directory = directory.path();

    let test_path = "test.R";
    let input = "if (TRUE) {\n1\n}\n";

    // Only a `.air.toml`, which is discovered just like a project `.air.toml`
    std::fs::write(
        user_directory.join(".air.toml"),
        "[format]\nindent-width = 8\n",
    )?;
    std::fs::write(directory.join(test_path), input)?;

    let output = Command::new(binary_path())
        .env(user_config_directory_env_var(), user_config_directory)
        .current_dir(directory)
        .arg("format")
        .arg(test_path)
        .run();
    assert!(output.status.success());
    assert_eq!(
        std::fs::read_to_string(directory.join(test_path))?,
        "if (TRUE) {\n        1\n}\n"
    );

    // Now add an `air.toml` as well, which is preferred over `.air.toml`
    std::fs::write(
        user_directory.join("air.toml"),
        "[format]\nindent-width = 4\n",
    )?;
    std::fs::write(directory.join(test_path), input)?;

    let output = Command::new(binary_path())
        .env(user_config_directory_env_var(), user_config_directory)
        .current_dir(directory)
        .arg("format")
        .arg(test_path)
        .run();
    assert!(output.status.success());
    assert_eq!(
        std::fs::read_to_string(directory.join(test_path))?,
        "if (TRUE) {\n    1\n}\n"
    );

    Ok(())
}

#[test]
fn test_user_config_unrooted_exclude() -> anyhow::Result<()> {
    let user_config_directory = TempDir::new()?;
    let user_config_directory = user_config_directory.path();
    write_user_air_toml(user_config_directory, "[format]\nexclude = [\"foo/\"]\n")?;

    let directory = TempDir::new()?;
    let directory = directory.path();

    std::fs::create_dir(directory.join("foo"))?;
    let excluded_path = Path::new("foo").join("a.R");
    let excluded_contents = "1+1";
    std::fs::write(directory.join(&excluded_path), excluded_contents)?;

    let formatted_path = "b.R";
    let formatted_contents = "1+1";
    std::fs::write(directory.join(formatted_path), formatted_contents)?;

    let output = Command::new(binary_path())
        .env(user_config_directory_env_var(), user_config_directory)
        .current_dir(directory)
        .arg("format")
        .arg(".")
        .run();

    assert!(output.status.success());

    // The user level `exclude` skips `foo/a.R` but not `b.R`
    assert_eq!(
        std::fs::read_to_string(directory.join(&excluded_path))?,
        excluded_contents
    );
    assert!(formatted_contents != std::fs::read_to_string(directory.join(formatted_path))?);

    Ok(())
}

#[test]
fn test_user_config_rooted_exclude_is_a_hard_error() -> anyhow::Result<()> {
    let user_config_directory = TempDir::new()?;
    let user_config_directory = user_config_directory.path();

    // A user level `exclude` must be unrooted, so a rooted pattern (interior `/`) is a
    // hard error.
    write_user_air_toml(
        user_config_directory,
        "[format]\nexclude = [\"src/foo.R\"]\n",
    )?;

    let directory = TempDir::new()?;
    let directory = directory.path();
    std::fs::write(directory.join("test.R"), "1+1")?;

    insta::assert_snapshot!(
        Command::new(binary_path())
            .env(user_config_directory_env_var(), user_config_directory)
            .current_dir(directory)
            .arg("format")
            .arg(".")
            .arg("--no-color")
            .run()
            .remove_arguments()
            .replace_stderr(
                &user_air_toml(user_config_directory).display().to_string(),
                "[AIR_TOML]"
            )
    );

    Ok(())
}

#[test]
fn test_user_config_broken_toml_is_a_hard_error() -> anyhow::Result<()> {
    let user_config_directory = TempDir::new()?;
    let user_config_directory = user_config_directory.path();
    write_user_air_toml(user_config_directory, "not valid toml")?;

    let directory = TempDir::new()?;
    let directory = directory.path();
    std::fs::write(directory.join("test.R"), "1+1")?;

    // A broken user level `air.toml` is a hard error, matching project `air.toml`
    insta::assert_snapshot!(
        Command::new(binary_path())
            .env(user_config_directory_env_var(), user_config_directory)
            .current_dir(directory)
            .arg("format")
            .arg(".")
            .arg("--no-color")
            .run()
            .replace_stderr(
                &user_air_toml(user_config_directory).display().to_string(),
                "[AIR_TOML]"
            )
    );

    Ok(())
}
