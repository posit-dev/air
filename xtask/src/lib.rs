//! Codegen tools mostly used to generate ast and syntax definitions. Adapted from rust analyzer's codegen

pub mod accept;
pub mod glue;

use bpaf::Bpaf;
use std::{
    env,
    fmt::Display,
    path::{Path, PathBuf},
};

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub enum TaskCommand {
    /// Regenerates every corpus snapshot
    #[bpaf(command)]
    Accept,
}

pub use crate::glue::{pushd, pushenv};

pub use anyhow::{anyhow, bail, ensure, Context as _, Error, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Overwrite,
    Verify,
}

pub fn project_root() -> PathBuf {
    let manifest_dir =
        env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned());

    // Searched rather than counted, because `xtask` and `xtask/codegen` sit at
    // different depths
    Path::new(&manifest_dir)
        .ancestors()
        .find(|directory| {
            std::fs::read_to_string(directory.join("Cargo.toml"))
                .is_ok_and(|manifest| manifest.contains("[workspace]"))
        })
        .expect("a crate in this workspace runs this")
        .to_path_buf()
}

pub fn reformat(text: impl Display) -> Result<String> {
    reformat_without_preamble(text).map(prepend_generated_preamble)
}

pub fn reformat_with_command(text: impl Display, command: impl Display) -> Result<String> {
    reformat_without_preamble(text).map(|formatted| {
        format!("//! This is a generated file. Don't modify it by hand! Run '{command}' to re-generate the file.\n\n{formatted}")
    })
}

pub const PREAMBLE: &str = "Generated file, do not edit by hand, see `xtask/codegen`";
pub fn prepend_generated_preamble(content: impl Display) -> String {
    format!("//! {PREAMBLE}\n\n{content}")
}

pub fn reformat_without_preamble(text: impl Display) -> Result<String> {
    let _e = pushenv("RUSTUP_TOOLCHAIN", "stable");
    ensure_rustfmt()?;
    let output = run!(
        "rustfmt --edition 2024 --config newline_style=Unix";
        <text.to_string().as_bytes()
    )?;

    Ok(format!("{output}\n"))
}

pub fn ensure_rustfmt() -> Result<()> {
    let out = run!("rustfmt --version")?;
    if !out.contains("stable") {
        bail!(
            "Failed to run rustfmt from toolchain 'stable'. \
             Please run `rustup component add rustfmt --toolchain stable` to install it.",
        )
    }
    Ok(())
}
