# -----------------------------------------------------------------------------
# Basic positioning

# This should be formatted
1+1

# fmt: skip
1+1

1+1 # fmt: skip

# This should be formatted
1+1
# fmt: skip
NULL

# fmt: skip
# Interleaving comment
1+1

# fmt: skip

1+1

# This should be formatted
1+1

# -----------------------------------------------------------------------------
# Calls

# fmt: skip
fn(
  1+1, 2+2
)

fn(
  1+1, 2+2
) # fmt: skip

# Just this argument
fn(
    # fmt: skip
    1+1,
    2+2
)

# Just this argument
fn(
    1+1, # fmt: skip
    2+2
)

# Just this argument, which should be moved to its own line but left unformatted
fn(
  1+1, 2+2 # fmt: skip
)

# Aligned lists
# fmt: skip
list(
  this      = 1,
  that      = 2,
  thisthing = 3,
  thatthing = 4,
  andthis   = 5
)

# -----------------------------------------------------------------------------
# Tribble

# Important test case

# fmt: skip
tribble(
  ~a, ~b,
   1,  2
)

# -----------------------------------------------------------------------------
# Binary expression chains

# Skips everything in the chain
# fmt: skip
foo |>
  bar() |> baz()

foo |>
  # Just `bar()`, but `baz()` moves to its own line and is formatted
  # fmt: skip
  bar(a = 1,
    b = 2
  ) |> baz(a = 1,
    b = 2
  )

# -----------------------------------------------------------------------------
# Functions

# fmt: skip
# Everything in the function
function(){
    1+1
}

# fmt: skip
# Everything within the assignment expression of `<-`
fn<-function(){
    1+1
}

function(){
    # fmt: skip
    # Just this line
    1+1
    2+2
}

# -----------------------------------------------------------------------------
# Braced expressions

# fmt: skip
# Everything inside
{
    1+1
    2+2
}

{
    # fmt: skip
    # Just this line
    1+1
    2+2
}

# -----------------------------------------------------------------------------
# If statements and comments

# fmt: skip
if (TRUE) 1 # hi
