function() 1
function(a, b) 1

function(a_really_long_argument_name_to_break_on, and_here_is_another_one_please_break_me, and_this) 1

function(a_really_long_argument_name_to_break_on, and_this) a_really_long_argument_name_to_break_on

function(a = {
  1
}, b) {
  1
}

function() {
  # comment
}

function() # becomes leading on `1 + 1`
{
  1 + 1
}

function() # becomes leading on `1 + 1`
{
  # an inner comment
  1 + 1
}

function() # becomes dangling on the `{}`
{
}

function() # becomes dangling on the `{}`
{
  # an inner comment but empty `{}`
}

function() # becomes leading on `1 + 1`
  1 + 1

\(x, y) 1
