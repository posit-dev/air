//! Codegen tools for generating Syntax and AST definitions. Derived from Rust analyzer's codegen
//!
mod ast;
mod css_kinds_src;
mod formatter;
mod generate_macros;
mod generate_node_factory;
mod generate_nodes;
mod generate_nodes_mut;
mod generate_syntax_factory;
mod generate_syntax_kinds;
mod graphql_kind_src;
mod grit_kinds_src;
mod js_kinds_src;
mod json_kinds_src;
mod markdown_kinds_src;
mod r_kinds_src;
mod yaml_kinds_src;

mod generate_crate;
mod html_kinds_src;
mod kind_src;
mod language_kind;
mod parser_tests;
mod termcolorful;
mod unicode;

use bpaf::Bpaf;
use std::path::Path;

use xtask::{glue::fs2, Mode, Result};

pub use self::ast::generate_ast;
pub use self::formatter::generate_formatters;
pub use self::generate_crate::generate_crate;
pub use self::parser_tests::generate_parser_tests;
pub use self::unicode::generate_tables;

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
    /// Extracts parser inline comments into test files
    #[bpaf(command)]
    Test,
    /// Generates unicode table inside lexer
    #[bpaf(command)]
    Unicode,
    /// Runs ALL the codegen
    #[bpaf(command)]
    All,
    /// Creates a new crate
    #[bpaf(command, long("new-crate"))]
    NewCrate {
        /// The name of the crate
        #[bpaf(long("name"), argument("STRING"))]
        name: String,
    },
}
