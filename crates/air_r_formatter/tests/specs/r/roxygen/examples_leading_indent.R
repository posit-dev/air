#| [format]
#| roxygen-examples = true

# Call sits at the boundary and is left untouched
Widget <- R6::R6Class(
  "Widget",
  public = list(
    #' @examples
    #' some_function(argument_one, argument_two, argument_three, argument_fours)
    initialize = function() {
      NULL
    }
  )
)

# Call wraps
Widget <- R6::R6Class(
  "Widget",
  public = list(
    #' @examples
    #' some_function(argument_one, argument_two, argument_three, argument_four, a)
    initialize = function() {
      NULL
    }
  )
)
