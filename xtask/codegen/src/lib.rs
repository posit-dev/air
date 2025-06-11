//! Codegen tools for generating Syntax and AST definitions. Derived from Rust analyzer's codegen
//!
mod ast;
mod formatter;
mod generate_macros;
mod generate_node_factory;
mod generate_nodes;
mod generate_nodes_mut;
mod generate_syntax_factory;
mod generate_syntax_kinds;
mod r_json_schema;
mod r_kinds_src;

mod kind_src;
mod language_kind;
mod termcolorful;

use bpaf::Bpaf;
use std::path::Path;

use xtask::{glue::fs2, Mode, Result};

pub use self::ast::generate_ast;
pub use self::formatter::generate_formatters;
pub use self::r_json_schema::generate_json_schema;

pub enum UpdateResult {
    NotUpdated,
    Updated,
}

/// A helper to update file on disk if it has changed.
/// With verify = false,
pub fn update(path: &Path, contents: &str, mode: &Mode) -> Result<UpdateResult> {
    match fs2::read_to_string(path) {
        Ok(old_contents) if old_contents == contents => {
            return Ok(UpdateResult::NotUpdated);
        }
        _ => (),
    }

    if *mode == Mode::Verify {
        anyhow::bail!("`{}` is not up-to-date", path.display());
    }

    eprintln!("updating {}", path.display());
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs2::create_dir_all(parent)?;
        }
    }
    fs2::write(path, contents)?;
    Ok(UpdateResult::Updated)
}

pub fn to_capitalized(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub enum TaskCommand {
    /// Generates formatters for each language
    #[bpaf(command)]
    Formatter,
    /// Transforms ungram files into AST
    #[bpaf(command)]
    Grammar(Vec<String>),
    /// Runs ALL the codegen
    #[bpaf(command)]
    All,
    #[bpaf(command, long("json-schema"))]
    JsonSchema,
}
