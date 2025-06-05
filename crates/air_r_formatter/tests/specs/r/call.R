fn()
fn(a)

# ------------------------------------------------------------------------
# Holes

# Leading holes should hug the `(` token
fn(,)
fn(,,)

# Non-leading holes retain spaces because they are considered "weird"
# and we want them to stand out
fn(, a,)
fn(, a, , )
fn(a,,b,,)

fn(a_really_long_argument_here,,another_really_really_long_argument_to_test_this_feature,,)

# ------------------------------------------------------------------------
# Dots

fn(...)
fn(..., a = 1)
fn(a = 1, ... = 2)
fn(a = 1, another_really_really_long_argument_to_test_this_feature, a_really_long_argument_here, ...)

# ------------------------------------------------------------------------
# Dot dot i

fn(..1, ..2)
fn(..1 = 1, ..2 = 2)

# ------------------------------------------------------------------------
# `NULL` argument name (r-lib/tree-sitter-r#164)

fn(NULL = 1)
fn(NULL = 1,)
fn(NULL = )
fn(NULL = ,)

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
    body
})

# ------------------------------------------------------------------------
# User requested line break and leading holes

# Leading holes are "invisible" when determining user requested expansion
# These all expand
fn(,
  x = 1
)

fn(
  ,
  x = 1
)

fn(
  , x = 1
)

fn(
  ,, x = 1
)

# A comment connected to a hole prevents it from being a "leading hole",
# instead it just becomes part of the typical arguments list and expands
fn(
  # comment
  ,
  x = 1
)

fn(
  ,
  # comment
  ,
  x = 1
)

# ------------------------------------------------------------------------
# Comments "inside" holes

fn(# comment
  ,
)

fn(, # comment
)
fn(,
  # comment
)
fn(
  , # comment
)
fn(
  ,
  # comment
)

fn(, # comment
  ,
)
fn(,
  # comment
  ,
)
fn(
  , # comment
  ,
)
fn(
  ,
  # comment
  ,
)

fn(
  ,
  , # comment1
  # comment2
  ,
  x
)

# Trails `a`
fn(
  a, # comment
  ,
  b
)
# Trails `a` technically, but should stay on own line
fn(
  a,
  # comment
  ,
  b
)
# Trails `a`
fn(
  a, # comment
  # comment2
  ,
  b
)

# Special test - ensure this leads `b` rather than trails `a`
fn(
  ,
  a,
  , # comment
  b
)

# Both comments lead the hole
fn(# comment1
  # comment2
  ,
  x
)

# Comment leads hole
# Following token is `,`, preceding before hole is another hole
fn(
  a,
  , # comment
  ,
  b
)
fn(
  , # comment
  ,
  x
)

# Comment leads `{` but doesn't move inside it
fn(
  ,
  , # comment
  { 1 +  1 }
)

# A particular motivating case. Want trailing `,` commentB to stay on `b`.
list2(
  a, # commentA
  b, # commentB
)

# ------------------------------------------------------------------------
# Comments "after" holes

# Both get attached to `x`
# Following token isn't `,`, `)`, `]`, or `]]`, and following node is non-hole,
# so we attach to it
fn(
  ,
  , # comment
  x
)
fn(
  ,
  , # comment1
  # comment2
  x
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

# Collapses with empty braces
with(data, {
})

with(data, {
  # dangling
})

# Collapses with empty braces
fn({
})

fn({
  # dangling
})

fn({
  1 + 1
})

fn(a = { 1 + 1 })

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

# Best fitting is not used, the empty `{}` would never be expanded, so we don't
# require best fitting
map(xs, function(x) {})
map(xs_that_is_really_long_to_just_barely_pass_the_line_lengthhh, function(x) {})

# Best fitting is not used, the empty `{}` would (typically) never be expanded,
# so we don't require best fitting
map(xs, function(x) {{ x }})
map(xs_that_is_long_to_just_barely_pass_the_line_lengthhhhh, function(x) {{ x }})

# Best fitting is used to choose the most flat variant, and this stays
# as is
map(xs, function(x) x)

# Best fitting is used to choose the most expanded variant over
# the current middle variant, because the middle variant exceeds
# the line length
map(my_long_list_my_long_list_my_long_list_my_long_list_my_long_list_my_long_list, function(my_long_argument) {
  my_long_body_my_long_body_my_long_body_my_long_body_my_long_body
})

# Best fitting is used to choose the most expanded variant over
# the current middle variant. Remember no soft-indents are allowed in the
# `parameters` when looking at options for the middle variant, so the middle
# variant can't choose to put each parameter on its own line in the current
# form. Only the most expanded form can do that when fully breaking everything.
map(x, function(a, a_really_really_long_parameter, and_another_one_here_too_wow_this_is_long) {
  1
})

# Best fitting is used to choose most expanded here. Even with the middle
# variant, the first map() argument and the function signature would exceed the
# line length. Autobracing DOES NOT kick in as it expands in this case.
map(my_long_list_my_long_list_my_long_list_my_long_list, function(my_long_argument) my_long_argument)

# Best fitting is used to choose most expanded here. Even with the middle
# variant, the first map() argument and the function signature would exceed the
# line length. Autobracing DOES kick in as it expands in this case as it fully
# expands the function definition too to keep it within the line length.
map(my_long_list_my_long_list_my_long_list_my_long_list, function(my_long_argument) my_long_list_my_long_list_my_long_list_my_long_list_my_long_list)

# Best fitting is used to choose the middle variant over the current most flat
# form. The middle variant fits within the line length, so is preferred over
# the most expanded form.
map(xs, function(my_long_argument) my_long_argument + my_extra_long_extra_argument)

# Best fitting is used to choose the middle variant. The line break forces
# autobracing, which means the function definition breaks. That rules out
# the most flat variant, and then the middle variant is preferred over most
# expanded because it fits in the line length.
map(xs, function(x, option = "a")
  x
)

# Best fitting is attempted, but we bail early because the persistent line
# break causes the function definition `parameters` to expand, and if that
# happens we fully expand
map(xs, function(
  x, option = "a") {
  x
})

# Best fitting is not used here. The `xs` already has a line break, so
# we stay fully expanded. The function parameters are flattened though.
map(
  xs,
  function(x,
    option = "a"
  ) {
    x
  }
)

# Best fitting is used to choose the middle variant. This is not a persistent
# line break location, so the function `parameters` don't actually break here.
# And then we choose middle over most expanded because it fits within the line
# length.
map(xs, function(x,
  option = "a") {
  x
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

# Named argument keeps name (#42)
map(xs, .f = function(x) {
  x + 1
})

# ------------------------------------------------------------------------
# Comments: Trailing braced expression

# Comments anywhere on a trailing braced expression should refuse to group
# and  force expanded output. This avoids some idempotence issues, and
# grouping can't possibly be useful here anyways, as the comment will be
# in the way.
# This includes:
# - Comments attached to the `AnyRArgument` node itself
# - Comments on the `name` node of a `AnyRArgument::NamedRArgument` node
# - Comments on the `value` node of a `AnyRArgument::NamedRArgument` node

with(
  xs, # end-of-line
  expr = {
    x + 1
  }
)

with(
  xs,
  # own-line
  expr = {
    x + 1
  }
)

with(
  xs,
  expr # end-of-line
  = {
    x + 1
  }
)

with(
  xs,
  expr
  # own-line
  = {
    x + 1
  }
)

with(
  xs,
  expr = # end-of-line
  {
    x + 1
  }
)

with(
  xs,
  expr =
  # own-line
  {
    x + 1
  }
)

with(
  xs,
  expr =
  {
    x + 1
  } # end-of-line
)

with(
  xs,
  expr =
  {
    x + 1
  }
  # own-line
)

# ------------------------------------------------------------------------
# Comments: Trailing inline function

# Comments anywhere on a trailing inline function should refuse to group and
# force expanded output. This avoids some idempotence issues, and grouping
# can't possibly be useful here anyways, as the comment will be in the way.
# This includes:
# - Comments attached to the `AnyRArgument` node itself
# - Comments on the `name` node of a `AnyRArgument::NamedRArgument` node
# - Comments on the `value` node of a `AnyRArgument::NamedRArgument` node

fn(
  xs, # end-of-line
  f = function(x) {
    x + 1
  }
)

fn(
  xs,
  # own-line
  f = function(x) {
    x + 1
  }
)

fn(
  xs,
  f # end-of-line
  = function(x) {
    x + 1
  }
)

fn(
  xs,
  f
  # own-line
  = function(x) {
    x + 1
  }
)

fn(
  xs,
  f = # end-of-line
  function(x) {
    x + 1
  }
)

fn(
  xs,
  f =
  # own-line
  function(x) {
    x + 1
  }
)

fn(
  xs,
  f =
  function(x) {
    x + 1
  } # end-of-line
)

fn(
  xs,
  f =
  function(x) {
    x + 1
  }
  # own-line
)

# ------------------------------------------------------------------------
# Comments: Named arguments without a RHS

switch(
  name,
  one = , # Trailing, stays beside `one`
  two = , # Trailing, stays beside `two`
  three = 1,
  stop("oh no")
)

# This is enclosed by the `RNamedArgument` node, so it moves on top
fn(
  x,
  one # Moves above `one`
  = ,
  two = 2
)

# This is not enclosed by the `RNamedArgument` node because it only contains
# `one =` and stops at the end of the `=`. So it is considered trailing.
fn(
  x,
  one = # Trailing, stays beside `one`
  ,
  two = 2
)

# This is not enclosed by the `RNamedArgument` node because it only contains
# `one =` and stops at the end of the `=`. So it is considered trailing.
fn(
  x,
  one = # Trailing, stays beside `one`
)

# ------------------------------------------------------------------------
# Special - trailing curly-curly

# A curly-curly is not treated as groupable, even though it fits the
# criteria of "trailing braced expression"
fn(x, {{ var }})

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

# Due to holes not having tokens, we collapse full empty lines in them
fn(

  # comment1
  ,

  # comment2
  ,

  b
)

fn(
  ,

  # comment2
  ,

  b
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

# ------------------------------------------------------------------------
# Hugging calls - https://github.com/posit-dev/air/issues/21

# Motivating hugging cases
abort(glue::glue("Length implied by `dim`, {n_elements}, must match the length of `x`, {n_x}."))
abort(paste0("This is a section", and, "this is another section", "and this is a final section"))

# Single line
c(list(1))

# Persistent newline
c(
    list(1)
)

# Symbol: Line length expansion
c(list(foobarbafoobarbafoobarbafoobarbafoobarbafoobarbafoobarbafoobarbafoobarbazzzzzzzzzfoobarbaz))

# Call: Recursive hugging case, no breaks
c(list(foobarbafoobarbafoobarbafoobarbafoobarbafoobarbafoobarbafoobarbafoobarbazzzzzzzzzfoobarbaz()))

# Call: Recursive hugging case, inner arguments break
c(list(foobarbafoobarbafoobarbafoobarbafoobarbafoobarbafoobarbafoobarbafoobarbazzzzzzzzzfoobarbaz(1, 2)))

# Call: Recursive hugging case, persistent newlines
c(list(foobar(
  1,
  2
)))

# Named arguments prevent hugging
fn(name = foobarbafoobarbafoobarbafoobarbafoobarbafoobarbafoobarbafoobarbafoobarbazzzzzzzzzfoobarbaz(1, 2))

# Named arguments prevent hugging - motivating example
# - With the `mutate()`, we expect multiple key/value pairs so hugging
#   after the first one doesn't feel quite right, and actually makes it
#   a little difficult to add additional key/value pairs
# - With the `filter()`, we accept hugging here and justify it by saying
#   that you end up reading this as `filterany`, which is kind of nice
storms <- storms %>%
  mutate(name = if_else(str_sub(name, 1, 3) %in% c("AL0", "AL1"), name, str_to_title(name))) %>%
  filter(any(status %in% c("hurricane", "tropical storm", "tropical depression")))

# Sanity checks for comments

c(
    #foo
    list(
        1
    )
)

c(
    list(
        1
    )
    #foo
)

c(list(
    #foo
    1
))

c(list(
    #foo
    x = 1
))

c(list(
    x =
    #foo
         1
))

c(list(
    #foo
))

# Trailing comment of inner paren
c(list(
    1
) #foo
)

# Leading comment of outer paren
c(list(
  1
)
#foo
)

c(
    list(
        1
    ) #foo
)

# Leading holes
fn(,, paste0("This is a section", and, "this is another section", "and this is a final section"))

fn[,, paste0("This is a section", and, "this is another section", "and this is a final section")]

# Subsetting
foo(bar[
  1,
  2
])

foo[[bar(
  1,
  2
)]]

# Fits on one line
foo[[bar(1, 2)]]

# Persistent line
foo[[
bar(
  1,
  2
)]]

foo(
  #foo
  bar[
  1,
  2
])

foo(bar[
  1,
  2
]
#foo
)

foo(bar[
  1,
  2
  #foo
]
)

foo( bar[
  #foo
  1,
  2
]
)
