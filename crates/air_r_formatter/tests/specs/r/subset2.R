fn[[]]
fn[[a]]

# Inherits call-like behavior with trailing braced expressions
fn[[a = { 1 + 1 }]]
fn[["description", {
  1 + 1
}]]

# ------------------------------------------------------------------------
# Holes

# Leading holes should hug the `[[` token
fn[[,]]
fn[[,,]]

fn[[a,,b,,]]
fn[[a_really_long_argument_here,,another_really_really_long_argument_to_test_this_feature,,]]

# Holes are "invisible" when determining user requested expansion
# These all expand
fn[[,
  x = 1
]]
fn[[
  ,
  x = 1
]]
fn[[
  , x = 1
]]
fn[[
  ,, x = 1
]]

# ------------------------------------------------------------------------
# Dots

fn[[...]]
fn[[..., a = 1]]
fn[[a = 1, ... = 2]]
fn[[a = 1, another_really_really_long_argument_to_test_this_feature, a_really_long_argument_here, ...]]

# ------------------------------------------------------------------------
# Dot dot i

fn[[..1, ..2]]
fn[[..1 = 1, ..2 = 2]]

# ------------------------------------------------------------------------
# Comments

fn[[
  # dangling special case
]]

# ------------------------------------------------------------------------
# User requested line break

# A line break before the first argument forces expansion

# So this data dictionary stays expanded even though it fits on one line
df[[
  df$col > 1,
  c(2, 3)
]]

# This flattens to one line
df[[df$col > 1,
  c(2, 3)
]]

# This flattens to one line
df[[df$col > 1, c(2, 3)
]]

# Expanding the inner subset forces expansion of the outer subset
df[[df$col > 7, map[[
  names(df)
]]]]

# ------------------------------------------------------------------------
# Comments "after" holes

df[[,
  # comment
  x
]]
