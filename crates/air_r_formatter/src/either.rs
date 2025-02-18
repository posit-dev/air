use biome_formatter::{prelude::Formatter, Format, FormatResult};

/// An owned value that generically represents two `Format` types
///
/// Can be chained.
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<L, R, Context> Format<Context> for Either<L, R>
where
    L: Format<Context>,
    R: Format<Context>,
{
    fn fmt(&self, f: &mut Formatter<Context>) -> FormatResult<()> {
        match self {
            Either::Left(left) => left.fmt(f),
            Either::Right(right) => right.fmt(f),
        }
    }
}
