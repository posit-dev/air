//! Pratt parser restrictions
//!
//! A restriction changes the flow of the pratt loop based on the surrounding
//! context
//!
//! Modeled after a similar idea in rust-analyzer and rustc, except we don't
//! gather them into a struct because we felt like managing the struct was more
//! complex than managing individual fields.
//! https://github.com/rust-lang/rust-analyzer/blob/9e0d1ec24eb81de12160ecfa7f31c89735bda951/crates/parser/src/grammar/expressions.rs#L188
//! https://github.com/rust-lang/rust/blob/92b5b8c87c61b20adc9001b41baf587634a74734/compiler/rustc_parse/src/parser/mod.rs#L77

/// Deny binary `=` assignment
///
/// `gram.y` has the concept of:
/// - `expr`
/// - `expr_or_help`
/// - `expr_or_assign_or_help`
///
/// It switches between `expr_or_help` and `expr_or_assign_or_help` depending on
/// whether or not binary `=` assign is allowed in the current context. This
/// isn't something that simple [crate::syntax::binding_power::BindingPower] can
/// model on its own. We effectively need to remove
/// [crate::syntax::binding_power::BindingPower::BINARY_EQUALS_ASSIGN] from the
/// binding power hierarchy altogether any time `expr_or_help` is used. We do that
/// with [BinaryEqualsAssign::Deny].
///
/// `expr_or_help` is used in the following places, where you will see explicit usage of
/// [BinaryEqualsAssign::Deny] for us:
/// - Call argument value
///   https://github.com/wch/r-source/blob/c92c5cb21a667bae2fa924fff2f8f2ceed005fb5/src/main/gram.y#L549-L556
/// - Function parameter default
///   https://github.com/wch/r-source/blob/c92c5cb21a667bae2fa924fff2f8f2ceed005fb5/src/main/gram.y#L559-L564
/// - For loop condition sequence
///   https://github.com/wch/r-source/blob/c92c5cb21a667bae2fa924fff2f8f2ceed005fb5/src/main/gram.y#L533
/// - While loop condition
///   https://github.com/wch/r-source/blob/c92c5cb21a667bae2fa924fff2f8f2ceed005fb5/src/main/gram.y#L527
/// - If condition
///   https://github.com/wch/r-source/blob/c92c5cb21a667bae2fa924fff2f8f2ceed005fb5/src/main/gram.y#L530
///
/// `expr_or_assign_or_help` is used in the following places, where you will see explicit
/// usage of [BinaryEqualsAssign::Allow] for us:
/// - Top level statement list
///   https://github.com/wch/r-source/blob/c92c5cb21a667bae2fa924fff2f8f2ceed005fb5/src/main/gram.y#L446-L450
/// - Braced statement list
///   https://github.com/wch/r-source/blob/c92c5cb21a667bae2fa924fff2f8f2ceed005fb5/src/main/gram.y#L537-L543
/// - Unary help operand
///   https://github.com/wch/r-source/blob/c92c5cb21a667bae2fa924fff2f8f2ceed005fb5/src/main/gram.y#L475
/// - Parenthesized expression body
///   https://github.com/wch/r-source/blob/c92c5cb21a667bae2fa924fff2f8f2ceed005fb5/src/main/gram.y#L469
/// - Function body
///   https://github.com/wch/r-source/blob/c92c5cb21a667bae2fa924fff2f8f2ceed005fb5/src/main/gram.y#L499-L501
/// - If / else body
///   https://github.com/wch/r-source/blob/c92c5cb21a667bae2fa924fff2f8f2ceed005fb5/src/main/gram.y#L503-L504
/// - For / while / repeat body
///   https://github.com/wch/r-source/blob/c92c5cb21a667bae2fa924fff2f8f2ceed005fb5/src/main/gram.y#L505-L507
#[derive(Debug, Clone, Copy)]
pub(crate) enum BinaryEqualsAssign {
    Allow,
    Deny,
}
