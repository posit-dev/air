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
