fn[]
fn[a]
fn[a = 1, ... = 2]

# Inherits call-like behavior with trailing braced expressions
fn[a = { 1 + 1 }]
fn["description", {
  1 + 1
}]

# TODO: Think about data.table usage, like:
DT[, {
  # write each group to a different file
  fwrite(.SD, "name")
}, by=x]

# This is pretty good
DT[, by=x, {
  # write each group to a different file
  fwrite(.SD, "name")
}]

# Holes
fn[,]
fn[,,]
fn[a,,b,,]
fn[a_really_long_argument_here,,another_really_really_long_argument_to_test_this_feature,,]

# Dots
fn[...]
fn[..., a = 1]
fn[a = 1, another_really_really_long_argument_to_test_this_feature, a_really_long_argument_here, ...]

# Comments
fn[
  # dangling special case
]
