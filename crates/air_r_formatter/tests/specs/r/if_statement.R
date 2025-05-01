# ---------------------------------------------------------------------------
# Comments

if (a) # becomes leading on `1 + 1`
{
  1 + 1
}

if (a) { # becomes leading on `1 + 1`
  1 + 1
}

if (a) # becomes dangling on `{}`
{
}

if (a) # becomes dangling on `{}`
{
  # inner comment but empty `{}`
}

if (a) # becomes leading on `TRUE`
  TRUE

if (
  a
  # becomes trailing on `a`
) {
  TRUE
}

if (a # becomes trailing on `a`
) {
  TRUE
}

{
  if (condition) this  # becomes trailing on `this`
  else that
}

{
  if (condition) this
  # becomes leading on `that`
  else that
}

{
  if (condition) {
    this
  } # becomes trailing on `this`
  else that
}

{
  if (condition) {

  } # becomes dangling on `{}`
  else that
}

{
  if (condition) {
    this
  }
  # becomes leading on `that`
  else that
}

{
  if (condition) {
    this
  }
  # becomes leading on `that`
  else {
    that
  }
}

{
  if (condition) {
    this
  }
  # becomes dangling on `{}`
  else {

  }
}

{
  if (condition) {
    this
  }
  # becomes leading on `that`
  else if (condition) {
    that
  }
}

{
  if (condition) {
    this
  }
  # becomes leading on `that`
  else if (condition) that
}

{
  if (condition) {
    this
  }
  # becomes dangling on `{}`
  else if (condition) {}
}

{
  if (condition) {
    this
  }
  # becomes leading on `that`
  # becomes leading on `that` part 2
  else if (condition) {
    that
  }
}

{
  if (condition) a
  else # becomes leading on `b`
    b
}

{
  if (condition) a
  else # becomes leading on `b`
    {
      b
    }
}

# Recursion into `consequence`
if (condition) if (condition2) this # becomes trailing on `this`
if (condition) if (condition2) if (condition3) this # becomes trailing on `this`
if (condition) if (condition2) this else that # becomes trailing on `that`

# Recursion into `alternative`
if (condition) this else that # becomes trailing on `that`
if (condition) this else if (condition2) that # becomes trailing on `that`
if (condition) this else if (condition2) this2 else that # becomes trailing on `that`
if (condition) this else if (condition2) this2 else if (condition3) that # becomes trailing on `that`

# ---------------------------------------------------------------------------
# Comments - these comments aren't "enclosed" by the if statement, but
# we attach EndOfLine comments with a preceding if statement node to the last
# node of the if statement to prepare for autobracing

# NOTE: Ideally these would stay as is and would not autobrace. That would work
# if we attached the comment as trailing on the if statement node. But because
# we attach it as trailing on `a` to have good behavior when autobracing, we
# end up forcing autobracing, because trailing end of line comments expand their
# parent group, which triggers the autobracing. That's ok by us, because overly
# autobracing is probably better than pooly placed comments.
if (condition) a # becomes trailing on `a`
if (condition) a else b # becomes trailing on `b`

if (condition) { a } # becomes trailing on `{}`
if (condition) { a } else { b } # becomes trailing on `{}`

if (condition) a_really_really_long_name_here_that_forces_a_break_because_it_is_so_long # becomes trailing on `a_really_really_long_name_here_that_forces_a_break_because_it_is_so_long`
if (condition) { a_really_really_long_name_here_that_forces_a_break_because_it_is_so_long } # becomes trailing on `{}`

if (condition) a_really_really_long_name_here else another_really_really_long_name # becomes trailing on `another_really_really_long_name`
if (condition) a_really_really_long_name_here else { another_really_really_long_name } # becomes trailing on `{}`

if (condition) a_really_really_long_name_here else if (condition2) another_really_really_long_name # becomes trailing on `another_really_really_long_name`
if (condition) a_really_really_long_name_here else if (condition2) { another_really_really_long_name } # becomes trailing on `{}`

if (condition) a_really_really_long_name_here else if (condition2) another_really_really_long_name else that_name # becomes trailing on `that_name`
if (condition) a_really_really_long_name_here else if (condition2) another_really_really_long_name else { that_name } # becomes trailing on `{}`

if (condition)
  a # becomes trailing on `a`

if (condition) # becomes leading on `a`
  a # becomes trailing on `a`

if (condition) {
  a
} # becomes trailing on `{}`

# We really want consistent behavior here, which takes some work.
# Comment on `a` is enclosed by the if statement, but comment on `b` is not!
{
  if (condition) a # becomes trailing on `a` 
  else           b # becomes trailing on `b`
}

{
  if (condition) a
  else           b # becomes trailing on `b`
}

{
  if (condition) a
  else {
    b
  } # stays trailing on `{}`
}

{
  if (condition) a
  else           b
  # Floating comment after the if statement
}

# ---------------------------------------------------------------------------
# Special

# Breaks, but the `condition` itself fits and is not expanded
if (this || this || this || this || this || this || this || this || this || this) {
  1
} else {
  2
}
# Breaks, but the `condition` itself also doesn't fit and is also expanded
if (this || this || this || this || this || this || this || this || this || this || this || this) {
  1
} else {
  2
}

# ---------------------------------------------------------------------------
# Auto bracing

# These are simple statements and are allowed on one line without braces
if (a) 1
if (a) 1 else 2
fn(if (a) 1)
fn(if (a) 1, if (a) 1 else 2)

# The group breaking forces braces
if (something_really_really_long_here_something_really_really_long_here) 1 else 2
if (a) something_really_really_long_here else and_something_really_really_long_here

# The leading newline forces braces
if (a)
  1
if (a) 1 else
  2

# The leading newline before `else` forces braces
{
  if (a) 1
  else 2
}

# The nested if forces braces
if (a) 1 else if (b) 2
if (a) 1 else if (b) 2 else 3

# The braces on one piece force braces
if (a) {1} else 2
if (a) {1} else {2}
