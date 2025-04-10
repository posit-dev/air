# Hard line break between `{}` tokens
function() {}
for (i in x) {}
if (a) {}

{}

{
  1
}

{
  # comment
}

# ------------------------------------------------------------------------
# Curly-curly

fn({{ var }})
fn({{ var }}, x, {{ var }})

# Part of more complex expression. Assume `fn()` enquos.
fn(mean({{ var }}))
fn({{ var }} + 1)

fn({{ var_that_is_extremely_long_and_eventually_forces_a_line_break_once_we_eventually_get_to_the_end }})

fn({{ # Leading of `var`
  var
}})

# Comprehensive comment test
fn(
# C1
{ # C2 (lifted up)
# C3 (lifted up)
{ # C4 (leads var)
  # C5 (leads var)
  var
  # C6
} # C7 (this line, but after folded 2nd `}`)
# C8 (after both `}}`)
} # C9 (same line as C8)
# C10
)

# Not curly-curly, not a symbol
fn({{ 1 }})
fn({{ (var) }})

# Not curly-curly, not inside an argument
{{ var }}
function(a = {{ var }}) {}

# Not curly-curly, 2 inner expressions
fn({{
  1
  2
}})

# Not curly-curly, 2 outer expressions
fn({
  { foo }
  bar
})

# Not curly-curly, 0 inner expressions
fn({{ }})

# Not curly-curly, 0 inner expressions (important, even with dangling comment!)
fn({{
  # dangling
}})
