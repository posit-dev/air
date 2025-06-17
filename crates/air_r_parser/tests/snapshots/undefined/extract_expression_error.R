# TODO: tree-sitter-r currently allows an `optional()` RHS, but our grammar
# requires a RHS, so we get `missing (required)`. These should eventually be
# parse errors if we can switch tree-sitter-r away from using `optional()`.
# For example, `a$1` gives:

#> RExtractExpression {
#>     left: RDoubleValue {
#>         value_token: R_DOUBLE_LITERAL@10..16 "1" [Newline("\n"), Whitespace("    ")] [],
#>     },
#>     operator: DOLLAR@16..17 "$" [] [],
#>     right: missing (required),
#> },
#> RDoubleValue {
#>     value_token: R_DOUBLE_LITERAL@17..18 "2" [] [],
#> },

a$1
a$NA
a$NULL
a$TRUE
a$(b)
a${ b }

a@1
a@NA
a@NULL
a@TRUE
a@(b)
a@{ b }
