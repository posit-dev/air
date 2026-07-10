#| [format]
#| roxygen-examples = true

# A blank line between the block and the documented node is preserved
#' @examples
#' 1+1

foo <- function() {
  1
}

# Multiple blank lines collapse to a single blank line
#' @examples
#' 1+1



foo <- function() {
  1
}

# A blank line before a following non-roxygen comment is preserved
#' @examples
#' 1+1

# regular comment
foo <- function() {
  1
}
