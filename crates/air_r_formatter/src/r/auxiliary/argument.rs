use crate::prelude::*;
use air_r_syntax::RArgument;
use air_r_syntax::RArgumentFields;
use biome_formatter::write;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatRArgument;
impl FormatNodeRule<RArgument> for FormatRArgument {
    fn fmt_fields(&self, node: &RArgument, f: &mut RFormatter) -> FormatResult<()> {
        let RArgumentFields { name_clause, value } = node.as_fields();

        match (name_clause, value) {
            // Hole
            // `foo(,)`
            // `foo(value, )`
            // `foo(, value)`
            (None, None) => Ok(()),

            // Unnamed argument
            // `foo(value)`
            // `foo(value, value)`
            (None, Some(value)) => write!(f, [value.format()]),

            // Named argument without a value
            // We write a mandatory space as a signal that this is a fairly
            // weird a nonstandard thing to see.
            // `foo(name = )`
            // `foo(name = , value)`
            (Some(name_clause), None) => write!(f, [name_clause.format(), space()]),

            // Named argument with a value
            // `foo(name = value)`
            (Some(name_clause), Some(value)) => {
                write!(f, [name_clause.format(), space(), value.format()])
            }
        }
    }
}
