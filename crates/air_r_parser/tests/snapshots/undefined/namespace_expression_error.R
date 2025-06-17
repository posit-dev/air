# TODO: tree-sitter-r currently allows an `optional()` RHS, but our grammar
# requires a RHS, so we get `missing (required)`. These should eventually be
# parse errors if we can switch tree-sitter-r away from using `optional()`.
# For example, `a::1` gives:

#> RNamespaceExpression {
#>     left: RIdentifier {
#>         name_token: IDENT@0..6 "a" [Newline("\n"), Whitespace("    ")] [],
#>     },
#>     operator: COLON2@6..8 "::" [] [],
#>     right: missing (required),
#> },
#> RDoubleValue {
#>     value_token: R_DOUBLE_LITERAL@8..9 "1" [] [],
#> },

a::1
a::NA
a::NULL
a::TRUE
a::(b)
a::{ b }

a:::1
a:::NA
a:::NULL
a:::TRUE
a:::(b)
a:::{ b }
