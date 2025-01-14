use std::path::Path;
use std::path::PathBuf;

use air::args::Args;
use air::run;
use air::status::ExitStatus;
use clap::Parser;
use tempfile::TempDir;

#[test]
fn default_options() -> anyhow::Result<()> {
    let tempdir = TempDir::new()?;
    let temppath = tempdir.path().join("test.R");
    std::fs::write(
        &temppath,
        r#"
1 + 1
"#,
    )?;

    let args = Args::parse_from(["", "format", temppath.to_str().unwrap()]);
    let err = run(args)?;

    assert_eq!(err, ExitStatus::Success);
    Ok(())
}

#[test]
fn test_check_returns_cleanly_for_multiline_strings_with_crlf_line_endings() -> anyhow::Result<()> {
    let fixtures = path_fixtures();
    let path = fixtures.join("crlf").join("multiline_string_value.R");
    let path = path.to_str().unwrap();

    let args = Args::parse_from(["", "format", path, "--check"]);
    let err = run(args)?;

    assert_eq!(err, ExitStatus::Success);
    Ok(())
}

fn path_fixtures() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}
