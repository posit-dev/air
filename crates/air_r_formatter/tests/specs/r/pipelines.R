# FIXME: Should respect vertical alignment (persist the newline of the first RHS)
mtcars |>
  mutate(foo = 1) %>%
  filter(
    foo == 1,
    bar == 2,
  ) |>
  ggplot(
    argument_that_is_quite_long = argument_that_is_quite_long,
    argument_that_is_quite_long = argument_that_is_quite_long
  )


# RHS of assignment should stay on same line as the `<-` operator
name <- mtcars |>
  mutate(foo = 1) %>%
  filter(
    foo == 1,
    bar == 2,
  ) |>
  ggplot(
    argument_that_is_quite_long = argument_that_is_quite_long,
    argument_that_is_quite_long = argument_that_is_quite_long
  )

name = mtcars |>
  mutate(foo = 1) %>%
  filter(
    foo == 1,
    bar == 2,
  ) |>
  ggplot(
    argument_that_is_quite_long = argument_that_is_quite_long,
    argument_that_is_quite_long = argument_that_is_quite_long
  )

# ----------------------------------------------------------------------------------------
# Line break persistance

# If any of the pipes break, all should break
# (Note that it isn't legal R code to have a break before the pipe, so we don't test
# those cases)

df |> foo() |> bar() |> baz()

df |>
foo() |> bar() |> baz()

df |> foo() |>
bar() |> baz()

df |> foo() |> bar() |>
baz()

# Works with mixed binary operator types
df |>
foo() %>% bar() |> baz()

# One line
df |> ggplot() + geom_line() + geom_bar()

# Expansion requested
df |>
ggplot() + geom_line() + geom_bar()

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
