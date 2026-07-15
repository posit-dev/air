#| [format]
#| roxygen-examples = true
#| indent-style = "tab"

# Call sits at the boundary and is left untouched. Each tab is measured as
# `indent-width` (2) columns, which stays under the line length.
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

# Call wraps. Each tab is measured as `indent-width` (2) columns, which pushes
# us over the line length.
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
