//! This module defines the Concrete Syntax Tree used by Biome.
//!
//! The tree is entirely lossless, whitespace, comments, and errors are preserved.
//! It also provides traversal methods including parent, children, and siblings of nodes.
//!
//! This is a simple wrapper around the `rowan` crate which does most of the heavy lifting and is language agnostic.

use crate::{RRoot, RSyntaxKind};
use biome_rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct RLanguage;

impl Language for RLanguage {
    type Kind = RSyntaxKind;
    type Root = RRoot;
}

pub type RSyntaxNode = biome_rowan::SyntaxNode<RLanguage>;
pub type RSyntaxToken = biome_rowan::SyntaxToken<RLanguage>;
pub type RSyntaxElement = biome_rowan::SyntaxElement<RLanguage>;
pub type RSyntaxNodeChildren = biome_rowan::SyntaxNodeChildren<RLanguage>;
pub type RSyntaxElementChildren = biome_rowan::SyntaxElementChildren<RLanguage>;
pub type RSyntaxList = biome_rowan::SyntaxList<RLanguage>;
