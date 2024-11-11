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

# ----------------------------------------------------------------------------------------
# Binary expression conditionals in if statements

# It fits, nothing to do
if (long_conditional1 && long_conditional2) {
  1 + 1
}

# User requested break
if (long_conditional1
&& long_conditional2) {
  1 + 1
}

# User requested break, parentheses prevent further splitting
if (long_conditional1
&& (long_conditional2 || long_conditional3)) {
  1 + 1
}

if (long_conditional1 && long_conditional2 && long_conditional3 && long_conditional4 && long_conditional5) {
  1 + 1
}
