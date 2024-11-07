test_that("description", {
	1 + 1
})

extended_test_that("description", {
	1 + 1
})

test_that("description that is super long and actually exceeds the line limit but we arent going to break!", {
	1 + 1
})

test_that(desc = "description that is super long and actually exceeds the line limit but we arent going to break!", code = {
	1 + 1
})

# Opening brace is moved back onto the first line because this is a test call
test_that("description that is super long and actually exceeds the line limit but we arent going to break!",
{
	1 + 1
})

# Both arguments are reflowed because this is a test call
test_that(
  "description that is super long and actually exceeds the line limit but we arent going to break!", {
	1 + 1
})

# The first argument isn't a string, so this isn't special cased and
# it gets expanded
# TODO: The expansion doesn't look right though
test_that(identity("description that is super long and actually exceeds the line limit but we arent going to break!"), {
	1 + 1
})
