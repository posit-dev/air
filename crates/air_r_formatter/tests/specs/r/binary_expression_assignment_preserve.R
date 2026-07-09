#| [format]
#| assignment-style = "preserve"

# Both `<-` and `=` are left exactly as written, in every position

# Top level
x <- 1
x = 1

# Inside `{ }`
{
  x <- 1
  y = 2
}

# Function body
function(x) x <- 1
function(x) x = 1

# If statement consequence
if (cond) x <- 1
if (cond) x = 1

# Other assignment-ish operators are also left as is
x <<- 1
6 -> x
7 ->> x
quote(x := 1)
x ~ y
