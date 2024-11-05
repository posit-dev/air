fn()
fn(a)
fn(a = 1, ... = 2)

# Holes
fn(,)
fn(,,)
fn(a,,b,,)
fn(a_really_long_argument_here,,another_really_really_long_argument_to_test_this_feature,,)

# Dots
fn(...)
fn(..., a = 1)
fn(a = 1, another_really_really_long_argument_to_test_this_feature, a_really_long_argument_here, ...)

# Trailing braced expression
test_that("description", {
	1 + 1
})

test_that("description that is super long and actually exceeds the line limit but we arent going to break!", {
	1 + 1
})

# TODO: This one should probably align the `{` on the opening line
# (all arguments before the trailing braced expression are on the opening line)
test_that("description that is super long and actually exceeds the line limit but we arent going to break!",
{
	1 + 1
})

# User broke it manually
test_that(
  "description that is super long and actually exceeds the line limit but we arent going to break!", {
	1 + 1
})

fn(a = { 1 + 1 })
