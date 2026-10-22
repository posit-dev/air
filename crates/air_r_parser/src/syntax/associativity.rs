/// Binary operator associativity, taken from `gram.y`
#[derive(Debug, Clone, Copy)]
pub(crate) enum Associativity {
    Left,
    Right,
}

impl Associativity {
    pub(crate) const BINARY_HELP: Self = Self::Left;
    pub(crate) const BINARY_LEFT_ASSIGN: Self = Self::Right;
    pub(crate) const BINARY_EQUALS_ASSIGN: Self = Self::Right;
    pub(crate) const BINARY_RIGHT_ASSIGN: Self = Self::Left;
    pub(crate) const BINARY_TILDE: Self = Self::Left;
    pub(crate) const BINARY_OR: Self = Self::Left;
    pub(crate) const BINARY_AND: Self = Self::Left;
    pub(crate) const BINARY_COMPARISON: Self = Self::Left;
    pub(crate) const BINARY_PLUS_MINUS: Self = Self::Left;
    pub(crate) const BINARY_MULTIPLY_DIVIDE: Self = Self::Left;
    pub(crate) const BINARY_SPECIAL_OR_PIPE: Self = Self::Left;
    pub(crate) const BINARY_COLON: Self = Self::Left;
    pub(crate) const BINARY_EXPONENTIATE: Self = Self::Right;
}
