1 + 2 + 3 + 4

argument_that_is_quite_long + argument_that_is_quite_long + argument_that_is_quite_long + argument_that_is_quite_long

1 + 2^3 + 4

argument_that_is_quite_long + argument_that_is_quite_long^argument_that_is_quite_long + argument_that_is_quite_long

1 + # comment
  2

1 +

  # comment1

  2 # comment2

# The following expressions should have spaces
1 ~ 2
1 <- 2
1 <<- 2
1 := 2
1 -> 2
1 ->> 2
1 = 2
1 | 2
1 & 2
1 || 2
1 && 2
1 < 2
1 <= 2
1 > 2
1 >= 2
1 == 2
1 != 2
1 + 2
1 - 2
1 * 2
1 / 2
1 |> 2
1 %>% 2

# The following expressions should not have spaces
1 ? 2
1 ** 2
1 ^ 2
1 : 2

# The following assignments should start the LHS/RHS on the same
# line as the operator
fn = function(x) {
  x
}
fn <- function(x) {
  x
}
fn <<- function(x) {
  x
}

identity(1) -> x
identity(1) ->> x

# -----------------------------------------------------------------------------
# Help specific

alias?"^try"
alias??"^try"
alias???"^try"
# Contact the oracle :)
alias????"^try"

# -----------------------------------------------------------------------------
# Precedence

a ~ b

# User requested line break
a ~
	b

# `b + c` is kept together. Precedence groups this as `(a) ~ (b + c)`,
# so the LHS of `~` doesn't contain another binary operator to chain to.
a ~
	b + c

# The two formulas are chained, but `c + d` stays together
a ~
	b ~ c + d

# `2 * 3` stay together as they are more tightly bound
1 +
	2 * 3

# And you keep this extra indent here if you force break
1 +
	2 *
    3

# TODO: Ideally the multiplication stay together on the same line,
# as biome's JS version does
a_really_really_long_thing_here1 * a_really_really_long_thing_here2 + a_really_really_long_thing_here3
a_really_really_long_thing_here1 + a_really_really_long_thing_here2 * a_really_really_long_thing_here3

# -----------------------------------------------------------------------------
# Binary expression conditionals in if statements

# It fits, nothing to do
if (long_conditional1 && long_conditional2) {
  1 + 1
}

# User requested break
if (long_conditional1 &&
long_conditional2) {
  1 + 1
}

# User requested break, parentheses prevent further splitting
if (long_conditional1 &&
(long_conditional2 || long_conditional3)) {
  1 + 1
}

# Not a user respected break because it comes before the `&&`,
# and we require it to come after
if (long_conditional1
&& long_conditional2) {
  1 + 1
}

if (long_conditional1 && long_conditional2 && long_conditional3 && long_conditional4 && long_conditional5) {
  1 + 1
}
