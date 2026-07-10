#| [format]
#| roxygen-examples = true

#' @examples
#'
#' 1+1 
foo <- function() {
  1
}

#' @examples
#' 
#'
#' 1+1
foo <- function() {
  1
}

# The blank lines after `@examplesIf` and `@examples` here are required for
# correct readibility
#' @examples
#' fn(1)
#' fn(2)
#' @examplesIf has_pkg()
#'
#' # `fn(2)` works with pkg
#' pkg::this(fn(2))
#' @examples
#'
#' # Another feature
#' another_demo(fn(3))
fn <- function(x) {
  x
}
