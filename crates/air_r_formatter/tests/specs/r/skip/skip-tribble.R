#' [format]
#' skip = ["tribble"]

# Should skip formatting even without a skip comment
tribble(
  ~x, ~y,
   1,  2,
   3,  4
)

# "Sees through" namespaces and looks at the function name
tibble::tribble(
  ~x, ~y,
   1,  2,
   3,  4
)

# Purposefully not smart enough to see through this
(tribble)(
  ~x, ~y,
   1,  2,
   3,  4
)

# Custom function would have to be added to the skip list to get skipped
tzibble <- function(...) {
    tribble(...)
}
tzibble(
  ~x, ~y,
   1,  2,
   3,  4
)
