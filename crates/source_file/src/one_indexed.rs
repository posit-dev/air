use std::fmt;
use std::fmt::Formatter;
use std::num::NonZeroUsize;
use std::num::ParseIntError;
use std::str::FromStr;

/// Type-safe wrapper for a value whose logical range starts at `1`, for
/// instance the line or column numbers in a file
///
/// Internally this is represented as a [`NonZeroUsize`], this enables some
/// memory optimizations
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct OneIndexed(NonZeroUsize);

impl OneIndexed {
    /// The largest value that can be represented by this integer type
    pub const MAX: Self = unwrap(Self::new(usize::MAX));
    // SAFETY: These constants are being initialized with non-zero values
    /// The smallest value that can be represented by this integer type.
    pub const MIN: Self = unwrap(Self::new(1));
    pub const ONE: NonZeroUsize = unwrap(NonZeroUsize::new(1));

    /// Creates a non-zero if the given value is not zero.
    pub const fn new(value: usize) -> Option<Self> {
        match NonZeroUsize::new(value) {
            Some(value) => Some(Self(value)),
            None => None,
        }
    }

    /// Construct a new [`OneIndexed`] from a zero-indexed value
    pub const fn from_zero_indexed(value: usize) -> Self {
        Self(Self::ONE.saturating_add(value))
    }

    /// Returns the value as a primitive type.
    pub const fn get(self) -> usize {
        self.0.get()
    }

    /// Return the zero-indexed primitive value for this [`OneIndexed`]
    pub const fn to_zero_indexed(self) -> usize {
        self.0.get() - 1
    }

    /// Saturating integer addition. Computes `self + rhs`, saturating at
    /// the numeric bounds instead of overflowing.
    #[must_use]
    pub const fn saturating_add(self, rhs: usize) -> Self {
        match NonZeroUsize::new(self.0.get().saturating_add(rhs)) {
            Some(value) => Self(value),
            None => Self::MAX,
        }
    }

    /// Saturating integer subtraction. Computes `self - rhs`, saturating
    /// at the numeric bounds instead of overflowing.
    #[must_use]
    pub const fn saturating_sub(self, rhs: usize) -> Self {
        match NonZeroUsize::new(self.0.get().saturating_sub(rhs)) {
            Some(value) => Self(value),
            None => Self::MIN,
        }
    }

    /// Checked addition. Returns `None` if overflow occurred.
    #[must_use]
    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        self.0.checked_add(rhs.0.get()).map(Self)
    }

    /// Checked subtraction. Returns `None` if overflow occurred.
    #[must_use]
    pub fn checked_sub(self, rhs: Self) -> Option<Self> {
        self.0.get().checked_sub(rhs.get()).and_then(Self::new)
    }
}

impl fmt::Display for OneIndexed {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        std::fmt::Debug::fmt(&self.0.get(), f)
    }
}

/// A const `Option::unwrap` without nightly features:
/// [Tracking issue](https://github.com/rust-lang/rust/issues/67441)
const fn unwrap<T: Copy>(option: Option<T>) -> T {
    match option {
        Some(value) => value,
        None => panic!("unwrapping None"),
    }
}

impl FromStr for OneIndexed {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OneIndexed(NonZeroUsize::from_str(s)?))
    }
}
