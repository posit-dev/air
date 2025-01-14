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
