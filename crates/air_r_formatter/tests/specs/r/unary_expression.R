+1
-1
?1
!1

+
1

# FIXME: The following newlines are preserved at a weird place
{
  + # Comment
  1

  + # Comment

  + # Comment

  1
}

1 + ++1

++argument_that_is_really_really_really_really_really_really_really_really_really_long

# ----------------------------------------------------------------------------
# Unary formulas (i.e. anonymous functions)

# Simple identifiers don't have a space between `~` and `foo`
~foo

# But anything else does have a space
~1
~.x + .y
~"foo"
~NULL

# This counts as an identifier
~.

# Removes line break
~
foo
~
1

# Chained formulas
~~foo
~~1 + 2
1~2
1~~2
1~~foo
