#| [format]
#| roxygen-examples = true

# Want to preserve exactly 1 empty line here. We have special code to preserve
# the `first` and `last` empty blank lines if they exist, but if they correspond
# to the same line, we don't want to preserve it twice!
#' @examples
#'
#' @returns
#' Something
foo <- function() {
  1
}
