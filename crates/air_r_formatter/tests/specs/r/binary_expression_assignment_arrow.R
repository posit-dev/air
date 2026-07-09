#| [format]
#| assignment-style = "arrow"

# Top level
x = 1

# A trailing comment on the operator is preserved
x = # keep me
  1

# Inside `{ }`
{
  x = 1
  y = 2
}

# Isolated by an extra pair of `( )`
(x = 1)

# Unary `?`
? x = 1
f(? x = 1)

# Inside binary `=` (both convert)
x = y = 1

# NOT inside binary `<-` as this is a syntax error to begin with!
# x <- y = 1

# Inside binary `?`
x = 1 ? y
y ? x = 1

# Function body
function(x) x = 1
lapply(xs, function(x) x = 1)

# If statement consequence
if (cond) x = 1

# Else clause alternative
if (cond) x else y = 1

# For loop body
for(i in 1:5) x = 1

# While loop body
while(cond) x = 1

# Repeat loop body
repeat x = 1

# Other assignment-ish operators are never changed
x <<- 1
6 -> x
7 ->> x
quote(x := 1)
x ~ y
