use biome_rowan::Language;
use biome_rowan::SyntaxNode;
use comments::Directive;

/// Generic trait for extracting [comments::Directive]s from a node's comments
pub trait CommentDirectives {
    type Language: Language;

    /// Returns an iterator over [comments::Directive]s present in a node's comments
    fn directives(&self, node: &SyntaxNode<Self::Language>) -> impl Iterator<Item = Directive>;
}
