#| [format]
#| roxygen-examples = true

# There is a trailing space after `interactive()` that must be removed
#' @examplesIf interactive() 
#' 1 + 1
foo <- function() {
  1
}
