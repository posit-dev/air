#| [format]
#| roxygen-examples = true
#| indent-style = "tab"

# The wrapped `@examples` code is always indented with spaces, even though the
# user requests tabs for the surrounding file
#' @examples
#' some_function(argument_one, argument_two, argument_three, argument_four, arg5)
foo <- function() {
  1
}
