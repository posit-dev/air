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

# -----------------------------------------------------------------------------
# Help specific

alias?"^try"
alias??"^try"
alias???"^try"
# Contact the oracle :)
alias????"^try"

# -----------------------------------------------------------------------------
# Non-chaining

# Soft line breaks kick in for long expressions
a_really_really_really_really_really_really_long_thing_here > a_really_really_really_really_really_really_long_thing_here2
a_really_really_really_really_really_really_long_thing_here >= a_really_really_really_really_really_really_long_thing_here2
a_really_really_really_really_really_really_long_thing_here < a_really_really_really_really_really_really_long_thing_here2
a_really_really_really_really_really_really_long_thing_here <= a_really_really_really_really_really_really_long_thing_here2
a_really_really_really_really_really_really_long_thing_here == a_really_really_really_really_really_really_long_thing_here2
a_really_really_really_really_really_really_long_thing_here != a_really_really_really_really_really_really_long_thing_here2

# Chaining does not occur
a_really_really_long_thing_here1 > a_really_really_long_thing_here2 > a_really_really_long_thing_here3

# Chaining occurs by chance due to how precedence naturally groups these
# along with having extremely long names
a_really_really_long_thing_here1 > a_really_really_really_really_really_really_long_thing_here2 > a_really_really_long_thing_here3

# Forced groups
(a_really_really_long_thing_here1 > a_really_really_long_thing_here2) > a_really_really_long_thing_here3
a_really_really_long_thing_here1 > (a_really_really_long_thing_here2 > a_really_really_long_thing_here3)

# Mixing with a chaining operator
a_really_really_long_thing_here1 > a_really_really_long_thing_here2 + a_really_really_long_thing_here3 + a_really_really_long_thing_here4
a_really_really_long_thing_here1 + a_really_really_long_thing_here2 + a_really_really_long_thing_here3 > a_really_really_long_thing_here4

# Chaining operator with high precedence forces non-chaining operator to expand
# (to convey a reading order that actually matches execution order)
df |>
  fn() > x
df |>
  fn() >= x
df |>
  fn() < x
df |>
  fn() <= x
df |>
  fn() == x
df |>
  fn() != x

# Chaining operator with low precedence does not force non-chaining operator to
# expand (because non-chaining operator does happen first!)
df &
  fn() > x
df &
  fn() >= x
df &
  fn() < x
df &
  fn() <= x
df &
  fn() == x
df &
  fn() != x

# User requested line break not respected for non-chainable items
# (This is debatable, but I see no need to enable it right now)
a >
  b

a ==
  b

# -----------------------------------------------------------------------------
# Chaining

# Chaining same operator
a_really_really_long_thing_here1 + a_really_really_long_thing_here2 + a_really_really_long_thing_here3
a_really_really_long_thing_here1 - a_really_really_long_thing_here2 - a_really_really_long_thing_here3
a_really_really_long_thing_here1 * a_really_really_long_thing_here2 * a_really_really_long_thing_here3
a_really_really_long_thing_here1 / a_really_really_long_thing_here2 / a_really_really_long_thing_here3
a_really_really_long_thing_here1 & a_really_really_long_thing_here2 & a_really_really_long_thing_here3
a_really_really_long_thing_here1 | a_really_really_long_thing_here2 | a_really_really_long_thing_here3
a_really_really_long_thing_here1 && a_really_really_long_thing_here2 && a_really_really_long_thing_here3
a_really_really_long_thing_here1 || a_really_really_long_thing_here2 || a_really_really_long_thing_here3
a_really_really_long_thing_here1 |> a_really_really_long_thing_here2 |> a_really_really_long_thing_here3
a_really_really_long_thing_here1 %>% a_really_really_long_thing_here2 %>% a_really_really_long_thing_here3

# Chaining mixed operator, same precedence group
a_really_really_long_thing_here1 + a_really_really_long_thing_here2 - a_really_really_long_thing_here3
a_really_really_long_thing_here1 - a_really_really_long_thing_here2 + a_really_really_long_thing_here3
a_really_really_long_thing_here1 * a_really_really_long_thing_here2 / a_really_really_long_thing_here3
a_really_really_long_thing_here1 / a_really_really_long_thing_here2 * a_really_really_long_thing_here3
a_really_really_long_thing_here1 |> a_really_really_long_thing_here2 %>% a_really_really_long_thing_here3
a_really_really_long_thing_here1 %>% a_really_really_long_thing_here2 |> a_really_really_long_thing_here3

# TODO: Do we really want these chained? It seems like no good can come of that even though they have same precedence
a_really_really_long_thing_here1 & a_really_really_long_thing_here2 && a_really_really_long_thing_here3
a_really_really_long_thing_here1 | a_really_really_long_thing_here2 || a_really_really_long_thing_here3

# Continuous chaining as long as precedence is high -> low from left -> right
# (e.g. `*` > `+` in terms of precedence, so keep chaining)
a_really_really_long_thing_here1 * a_really_really_long_thing_here2 + a_really_really_long_thing_here3
a_really_really_long_thing_here1 & a_really_really_long_thing_here2 | a_really_really_long_thing_here3
a_really_really_long_thing_here1 && a_really_really_long_thing_here2 || a_really_really_long_thing_here3
a_really_really_long_thing_here1 %>% a_really_really_long_thing_here2 + a_really_really_long_thing_here3
a_really_really_long_thing_here1 %>% a_really_really_long_thing_here2 * a_really_really_long_thing_here3

# Chaining breaks when precedence goes from low -> high at any point from
# left -> right (e.g. `+` < `*` in terms of precedence, so we break)
a_really_really_long_thing_here1 + a_really_really_long_thing_here2 * a_really_really_long_thing_here3
a_really_really_long_thing_here1 | a_really_really_long_thing_here2 & a_really_really_long_thing_here3
a_really_really_long_thing_here1 || a_really_really_long_thing_here2 && a_really_really_long_thing_here3
a_really_really_long_thing_here1 + a_really_really_long_thing_here2 %>% a_really_really_long_thing_here3
a_really_really_long_thing_here1 * a_really_really_long_thing_here2 %>% a_really_really_long_thing_here3

# Chaining breaks when precedence goes from low -> high at any point from
# left -> right (e.g. `+` < `*` in terms of precedence, so we break)
# AND we get a second indent if the sub group also breaks
a_really_really_long_thing_here1 + a_really_really_long_thing_here2 * a_really_really_really_really_really_really_really_really_long_thing_here3
a_really_really_long_thing_here1 | a_really_really_long_thing_here2 & a_really_really_really_really_really_really_really_really_long_thing_here3
a_really_really_long_thing_here1 || a_really_really_long_thing_here2 && a_really_really_really_really_really_really_really_really_long_thing_here3
a_really_really_long_thing_here1 + a_really_really_long_thing_here2 %>% a_really_really_really_really_really_really_really_really_long_thing_here3
a_really_really_long_thing_here1 * a_really_really_long_thing_here2 %>% a_really_really_really_really_really_really_really_really_long_thing_here3

# Chaining fully expands when there is a user requested line break
# in the higher precedence subgroup
a + b *
  c
a | b &
  c
a || b &&
  c
a + b %>%
  c
a * b %>%
  c

# Chaining doesn't fully expand when there is a user requested line break
# in the lower precedence subgroup
a +
  b * c
a |
  b & c
a ||
  b && c
a +
  b %>% c
a *
  b %>% c

# With sticky operators
a_really_really_long_thing_here1 + a_really_really_long_thing_here2 ** a_really_really_long_thing_here3
a_really_really_long_thing_here1 ** a_really_really_long_thing_here2 + a_really_really_long_thing_here3
a_really_really_long_thing_here1 * a_really_really_long_thing_here2 ** a_really_really_long_thing_here3
a_really_really_long_thing_here1 ** a_really_really_long_thing_here2 * a_really_really_long_thing_here3
a_really_really_long_thing_here1 + a_really_really_long_thing_here2 : a_really_really_long_thing_here3
a_really_really_long_thing_here1 : a_really_really_long_thing_here2 + a_really_really_long_thing_here3
a_really_really_long_thing_here1 * a_really_really_long_thing_here2 : a_really_really_long_thing_here3
a_really_really_long_thing_here1 : a_really_really_long_thing_here2 * a_really_really_long_thing_here3

# The `*` in the middle splits the `+` chain
a_really_really_long_thing_here1 + a_really_really_long_thing_here2 * a_really_really_long_thing_here3 + a_really_really_long_thing_here1

# The user line break after the `*` is respected
# Note how we use indents to show how the LHS/RHS of `*`
# are grouped even with the user line break, and then dedent
# before printing `a_really_really_long_thing_here4`
a_really_really_long_thing_here1 + a_really_really_long_thing_here2 *
  a_really_really_long_thing_here3 + a_really_really_long_thing_here4

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

# -----------------------------------------------------------------------------
# Line break persistance

# If the user requests a line break after the first pipe, then they all break

df |> foo() |> bar() |> baz()

df |>
foo() |> bar() |> baz()

df |> foo() |>
bar() |> baz()

# Flattened, line break is not after the first pipe!
df |> foo() |> bar() |>
baz()

# Flattened, removing just this first persistent line break is how users can
# easily request flattened pipe chains if one is possible (as opposed
# to having to flatten every pipe element to keep it flat)
df |> foo() |>
  bar() |>
  baz()

# Works with mixed binary operator types
df |>
foo() %>% bar() |> baz()

# One line
df |> ggplot() + geom_line() + geom_bar()

# Expansion requested
df |>
ggplot() + geom_line() + geom_bar()

# Flattened, this is one big chain due to monotonically decreasing
# precedence, so user requested expansion only applies to 1st operator
df |> ggplot() +
  geom_line() + geom_bar()

# Flattened, this is one big chain due to monotonically decreasing
# precedence, so user requested expansion only applies to 1st operator
df |> ggplot() + geom_line() +
geom_bar()

# Non-binary operators break the expansion propagation
(df |> foo()) |>
bar() |>
baz() |>
{ . |> and() |> this() }

(df |> foo()) |>
bar() |>
baz() |>
{ . |>
and() |> this() }

(1 + 2 * 3) +
  (4 + 5 * 6) + (7 + 8)

# Sticky binary operators break the expansion propagation
# (`2:3` stays together, `6^7` stays together)
1 +
  2:3 +
  4 +
  5 +
  6^7 + 8 +
  9

# Precedence is taken into account correctly
1:2 + 3

1:2 +
3

# Inside parentheses, subset, or, subset2, you can put a newline before
# the `|>`, which isn't valid R code at top level. This doesn't result
# in a break because we strictly require the persistent line break to come
# AFTER the first binary operator in the chain.
(df
|> foo())

x[df
|> foo()]

x[[df
|> foo()]]

# This does retain the persistent line break, because it comes after the pipe
(df |>
foo())

x[df |>
foo()]

x[[df |>
foo()]]

# -----------------------------------------------------------------------------
# Blank lines between `operator` and `right`

# Retain at most 1 blank line between the `operator` and the `right`.
# This is the same behavior as retaining blank lines in top level expressions,
# and between sequential arguments in calls.
# This is common with pipelines.
df |>
  a() |>

  # Some important notes about this complex call
  b() |>


  # Some more important notes
  c() |>
  d()

# -----------------------------------------------------------------------------
# Comments in chains

df |> foo() # Trailing of `df |> foo()` pipe chain

# Leading of `df |> foo() |> bar() |> baz()` pipe chain
df |>
  foo() |>
  bar() |>
  baz()

df |>
  # Leading of `foo()` call
  foo() |>
  # Leading of `bar()` call
  bar() |>
  # Leading of `baz()` call
  baz()

df |> # Trailing of `df` identifier
  foo() |>
  bar() |>
  baz()

df |>
  foo() |> # Trailing of `df |> foo()` pipe chain
  bar() |>
  baz()

df |>
  foo() |>
  bar() |> # Trailing of `df |> foo() |> bar()` pipe chain
  baz()

df |>
  foo() |>
  bar() |>
  baz() # Trailing of `df |> foo() |> bar() |> baz()` pipe chain

# -----------------------------------------------------------------------------
# Mixing pipes and pluses

# i.e. piping into a ggplot2 chain, which gets special treatment

# We don't add an extra level of indent after the first `+`,
# it is specially treated as being within the same group as the `|>`
df |>
  ggplot() +
  geom_line() +
  geom_bar()

df %>%
  ggplot() +
  geom_line() +
  geom_bar()

# Piping OUT of a `+` chain should add an extra indent.
# This is illogical behavior, it results in `identity(geom_bar())`, which is
# definitely not what the user wants, so the extra indent is a good thing
# as it proves that you've entered a different "group".
ggplot() +
  geom_line() +
  geom_bar() %>%
    identity()

# -----------------------------------------------------------------------------
# Assignment

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

# Assignment comment tests
fn <- function(x) # comment1
  { # comment2
    x # comment3
  } # comment4

identity(1) -> x
identity(1) ->> x

# Assignment never automatically breaks around the operator itself
# (Persistent line breaks after the operator are respected though)
a_really_really_really_really_really_really_long_thing_here ~ a_really_really_really_really_really_really_long_thing_here2
a_really_really_really_really_really_really_long_thing_here = a_really_really_really_really_really_really_long_thing_here2
a_really_really_really_really_really_really_long_thing_here := a_really_really_really_really_really_really_long_thing_here2
a_really_really_really_really_really_really_long_thing_here <- a_really_really_really_really_really_really_long_thing_here2
a_really_really_really_really_really_really_long_thing_here -> a_really_really_really_really_really_really_long_thing_here2
a_really_really_really_really_really_really_long_thing_here <<- a_really_really_really_really_really_really_long_thing_here2
a_really_really_really_really_really_really_long_thing_here ->> a_really_really_really_really_really_really_long_thing_here2

# -----------------------------------------------------------------------------
# Assignment with persistent line breaks

# Persistent line break after left-ish assignment
fn ~
  value
fn =
  value
fn <-
  value
fn <<-
  value

# Important that comment3 trails `value` here!
fn <- # comment1
  # comment2
  value # comment3

# No persistent line break after walrus operator
fn :=
  value

# We want these to match, neither support persistent line breaks
call(fn :=
  value)
call(fn =
  value)

# No persistent line break after right assignment
fn ->
  value
fn ->>
  value

# https://github.com/posit-dev/air/issues/91
is_condition_true <-
  if (condition) {
    "yes"
  } else {
    "no"
  }

# https://github.com/posit-dev/air/issues/91
base_version <-
  version %||%
  b_get(brand, "defaults", "shiny", "theme", "version") %||%
  b_get(brand, "defaults", "bootstrap", "version") %||%
  version_default()


# https://github.com/posit-dev/air/issues/220
data <-
  starwars |>
  filter(height > 172) |>
  select(1:3)

plot <-
  ggplot() +
  geom_point()

foo <-
  1 +
  2

foo <-
  TRUE ||
  FALSE

# Persistent assign-newline and unbroken pipeline
data <-
  ggplot() + geom_point()

# -----------------------------------------------------------------------------
# Assignment-like tilde

# After getting some feedback and practical experience with it, we've realized
# that `~` should be treated like an assignment operator. It also makes sense
# because `~` is next to assignment operators in the precedence table (see
# `?Syntax`).

a~b

# Persistent line break
a~
b

a~
# comment
b

# Model formula examples
y ~ this + that + this_other_thing

y ~ this +
that + this_other_thing

y ~ this + that +
this_other_thing

# `this` is never automatically forced onto a new line, just like how `<-` works
y ~ this + that + this_other_thing + this_other_thing + this_other_thing + this_other_thing

# Model formulas use left-aligned chaining if you introduce a persistent line
# break before the first RHS element
y ~
  this +
  that +
  this_thing

# Nothing changes, assignment won't automatically break around the operator
a_really_really_long_thing_here1 ~ a_really_really_long_thing_here2 ~ a_really_really_long_thing_here3
a_really_really_long_thing_here1 ~ a_really_really_really_really_really_really_long_thing_here2 ~ a_really_really_long_thing_here3
(a_really_really_long_thing_here1 ~ a_really_really_long_thing_here2) ~ a_really_really_long_thing_here3
a_really_really_long_thing_here1 ~ (a_really_really_long_thing_here2 ~ a_really_really_long_thing_here3)

# RHS breaks first
some_long_expression + some_long_expression ~ some_long_expression + some_long_expression

# If RHS breaking doesn't fit in the line length, LHS also breaks
# (Like with other assignment, a line break after the `~` is never introduced
# by the formatter itself. A user must request this.)
some_really_long_expression + some_really_long_expression ~ some_really_long_expression + some_really_long_expression

# Persistent line breaks allow you to break these in a different way
some_long_expression + some_long_expression ~
  some_long_expression + some_long_expression
some_really_long_expression + some_really_long_expression ~
  some_really_long_expression + some_really_long_expression

# This can be useful with `case_when()` if you want to use persistent line
# breaks to align long two sided formulas with short two sided formulas
case_when(
  some_long_expression == some_long_expression ~
    some_long_expression + some_long_expression,
  some_short_expression ~
    some_consistently_indented_value
)

# Should stay as is (posit-dev/air#402)
case({
  (x == 1) ~ {
    print("x is 1")
  }
  (x == 2) ~ {
    print("x is 2")
  }
})

# Should use `ChainAlignment::LeftAligned` to ensure `partnered` and `sex2`
# are at the same level of alignment (posit-dev/air#336)
tobacco ~
  partnered +
  sex2 +
  s(year_interview, by = sex2) +
  s(age, by = sex2) +
  s(educationyears, by = sex2) +
  s(householdsize, by = sex2) +
  s(parity0, by = sex2)

# Should
# - Expand all arguments
# - Keep `AGE` on the line with the `~`
# - Left-align `SEX` and the remainder of the chain
# https://github.com/posit-dev/positron/discussions/6095#discussioncomment-13112983
foo <- cph(Surv(dm_py, diabetes) ~ AGE + SEX + SEX + SEX + SEX + SEX + SEX + SEX  + SEX  + SEX  + SEX  + SEX  + SEX  + SEX ,
  data = dt,
  x = TRUE,
  y = TRUE
)

# Taking (a shorter version of) the previous example, this is what old versions
# of Air would give. It should now adjust this to left-align `AGE` and `SEX`
# together.
foo <- cph(
  Surv(dm_py, diabetes) ~
    AGE +
      SEX +
      SEX,
  data = dt,
  x = TRUE,
  y = TRUE
)

# Should stay as is
# https://github.com/posit-dev/air/issues/336#issuecomment-3371589684
starwars |>
  mutate(replace_when(
    pick(species, name),
    homeworld == "Tatooine" ~ tibble(
      species = "Tatooinese",
      name = paste(name, "(Tatooine)")
    )
  ))

# -----------------------------------------------------------------------------
# Assignment-like tilde and comments

# comment1
y ~ x # comment2

y ~ # comment1
 # comment2
 x

y ~ x1 + x2 +
  x3 # comment

# The comment actually forces expansion
y ~ x1 + x2 + # comment1
  x3 # comment2
