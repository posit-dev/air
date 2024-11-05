fn()
fn(a)
fn(a = 1, ... = 2)

# TODO: the `}` should "hug" the `)`, the `a = {` should stay on same line
fn(a = { 1 + 1 })

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
