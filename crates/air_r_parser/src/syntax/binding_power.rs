/// How tightly an operator binds. Higher wins!
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct BindingPower(u8);

impl BindingPower {
    pub(crate) const LOWEST: Self = Self(0);

    // Taken from R's `gram.y`
    pub(crate) const BINARY_HELP: Self = Self(1);
    pub(crate) const UNARY_HELP: Self = Self(1);
    pub(crate) const FUNCTION_OR_LOOP_OR_IF: Self = Self(2);
    pub(crate) const BINARY_LEFT_ASSIGN: Self = Self(3);
    pub(crate) const BINARY_EQUALS_ASSIGN: Self = Self(4);
    pub(crate) const BINARY_RIGHT_ASSIGN: Self = Self(5);
    pub(crate) const BINARY_TILDE: Self = Self(6);
    pub(crate) const UNARY_TILDE: Self = Self(6);
    pub(crate) const BINARY_OR: Self = Self(7);
    pub(crate) const BINARY_AND: Self = Self(8);
    pub(crate) const UNARY_NOT: Self = Self(9);
    pub(crate) const BINARY_COMPARISON: Self = Self(10);
    pub(crate) const BINARY_PLUS_MINUS: Self = Self(11);
    pub(crate) const BINARY_MULTIPLY_DIVIDE: Self = Self(12);
    pub(crate) const BINARY_SPECIAL_OR_PIPE: Self = Self(13);
    pub(crate) const BINARY_COLON: Self = Self(14);
    pub(crate) const UNARY_PLUS_MINUS: Self = Self(15);
    pub(crate) const BINARY_EXPONENTIATE: Self = Self(16);

    pub(crate) const HIGHEST: Self = Self(17);

    /// Used by left associative constructs
    pub(crate) fn increment(self) -> Self {
        Self(self.0 + 1)
    }
}
