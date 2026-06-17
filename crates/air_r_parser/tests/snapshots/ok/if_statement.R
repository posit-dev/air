if (a) 1
if (a) 1 else 2
if (a) 1 else if (b) 2 else 3

if # comment1
  (a) # comment2
  1 else # comment3
  2 # comment4

# ------------------------------------------------------------------------------
# `else`-like identifier is not treated as `else` (#499)

# ASCII letter
{
  if (TRUE) 1
  elseidx <- 1
}

# ASCII number
{
  if (TRUE) 1
  else1idx <- 1
}

# ASCII `_`
{
  if (TRUE) 1
  else_idx <- 1
}

# ASCII `.`
{
  if (TRUE) 1
  else.idx <- 1
}

# Non-ASCII, but considered `iswalnum()` in R with UTF-8 locale.
# Not considered `iswalnum()` in our scanner with C locale, but
# our overapproximation with `>= 128` allows this and aligns with R.
{
  if (TRUE) 1
  elseμ <- 1
}

# Non-ASCII (U+00B7), rejected by R as syntax error since `iswalnum()` returns false.
# Our overapproximation with `>= 128` allows this, but that's ok since the R
# side behavior is a syntax error.
{
  if (TRUE) 1
  else· <- 1
}