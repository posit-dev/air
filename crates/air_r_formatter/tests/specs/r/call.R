fn()
fn(a)
fn(a = 1, ... = 2)

# ------------------------------------------------------------------------
# Holes

fn(,)
fn(,,)
fn(a,,b,,)
fn(a_really_long_argument_here,,another_really_really_long_argument_to_test_this_feature,,)

# ------------------------------------------------------------------------
# Dots

fn(...)
fn(..., a = 1)
fn(a = 1, another_really_really_long_argument_to_test_this_feature, a_really_long_argument_here, ...)

# ------------------------------------------------------------------------
# User requested line break

# A line break before the first argument forces expansion

# So this data dictionary stays expanded even though it fits on one line
dictionary <- list(
  a = 1,
  b = 2
)

# This flattens to one line
dictionary <- list(a = 1,
  b = 2
)

# This flattens to one line
dictionary <- list(a = 1, b = 2
)

# Expanding the inner list forces expansion of the outer list
list(a = 1, b = list(
  foo = "a", bar = "b"))

# Expansion of `bar()` forces expansion of the whole pipeline
# (But note `foo(a = 1)` is not expanded)
df |> foo(a = 1) |> bar(
  b = 2, c = 3)

# Expansion of `foo()` forces expansion of the whole pipeline
# (But note `bar(b = 2, c = 3)` is not expanded)
df |> foo(
  a = 1) |> bar(b = 2, c = 3)

# Test-like call overrides user requested line break
# (test-like check comes first and seems more relevant)
test_that(
  "description", {

})

# TODO: Holes currently don't force expansion. There is no token attached to
# `RHoleArgument`, so we can't compute the "number of lines before the first token".
fn(
  ,
  x = 1
)

# ------------------------------------------------------------------------
# Trailing braced expression

with(data, {
  col
})

with(data,
  {
    col
  }
)

# User requested line break before `data` is respected
with(
  data,
  {
    col
  }
)

# User requested line break before `data` is respected
with(
  data,
  # A comment
  {
    col
  }
)

with(data, # Prevents flattening
	{
		col
	}
)

with(data,
  expr = {
    col
  }
)

with(data,
  foo = "bar",
  {
    col
  }
)

# Not trailing, stays expanded
with(data,
  {
    col
  },
  foo = "bar"
)

# Breaks and fully expands due to line length
with(my_long_list_my_long_list_my_long_list_my_long_list_long_long_long_long_long_list,
  {
    col
  }
)

with(data, {
})

with(data, {
  # dangling
})

fn({
})

fn({
  # dangling
})

fn({
  1 + 1
})

fn(a = { 1 + 1 })

fn({
  {
    1 + 1
  }
})

# The first argument here breaks, causing everything to fully expand
fn({ 1 + 1 }, {
	1 + 1
})

# Hole prevents `{` from looking like the last expression, so everything expands
fn(x, { 1 + 1 }, )

# ------------------------------------------------------------------------
# Trailing inline function

map(xs, \(x) {
  x + 1
})

map(xs, function(x) {
  x + 1
})

# Braces expand over multiple lines
map(xs, function(x) { })

# This should stay where it is
map(xs, function(x) x)

# This form is too wide, so it fully expands
map(my_long_list_my_long_list_my_long_list_my_long_list, function(my_long_argument) {
  my_long_body_my_long_body_my_long_body_my_long_body_my_long_body
})

# Parameter names are very long, so it fully expands
# (Note that this uses best-fitting. The `parameters` themselves don't force a
# break, but when a best-fit choice is made between this form with no
# soft-indents allowed in the `parameters` and the fully expanded form, the
# fully expanded form wins)
map(x, function(a, a_really_really_long_parameter, and_another_one_here_too_wow_this_is_long) {
  1
})

# The `{ 1 }` parameter would force a hard line break. We detect this and don't
# use best-fitting. Instead we fall back to the most expanded form.
map(x, function(a = { 1 }) {
  1
})

# Since there is only 1 argument, we want these to hug the function call even
# though the `parameters` cause a break and would typically force full expansion
fn(function(a = { 1 }) {
  1
})
fn(function(a, a_really_really_long_parameter, and_another_one_here_too_wow_this_is_long) {
  1
})

# ------------------------------------------------------------------------
# Empty lines between arguments

# 1 full empty line between sequential arguments is respected
# (like with top level expressions), but empty lines right after `(`
# and right before `)` are removed.
fn(

  a,

  b,


  c

)

fn(

  a,

  # comment1
  b,


  # comment2
  c
)

# ------------------------------------------------------------------------
# Comments

fn(
  # dangling special case
)

fn(
  a, # on line of `a`
  b
)

fn(
  # top of `a`
  a,
  b
)
