# FIXME: `a$1` currently produces:

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
