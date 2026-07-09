#| [format]
#| assignment-style = "equal"

# `<-` becomes `=` in all the positions where `=` is a valid assignment operator

# Top level
x <- 1

# A trailing comment is preserved
x <- # keep me
  1

# Inside `{ }`
{
  x <- 1
  y <- 2
}

# Isolated by an extra pair of `( )`
(x <- 1)

# Unary `?`
? x <- 1
f(? x <- 1)

# Inside binary `<-` (both convert)
x <- y <- 1

# Inside binary `=` (notably `x <- y = 1` is an R syntax error)
x = y <- 1

# Inside binary `?`
x <- 1 ? y
y ? x <- 1

# Function body
function(x) x <- 1
lapply(xs, function(x) x <- 1)

# If statement consequence
if (cond) x <- 1

# Else clause alternative
if (cond) x else y <- 1

# For loop body
for(i in 1:5) x <- 1

# While loop body
while(cond) x <- 1

# Repeat loop body
repeat x <- 1

# Other assignment-ish operators are never changed
x <<- 1
6 -> x
7 ->> x
quote(x := 1)
x ~ y

# `<-` is left untouched where `=` cannot replace it

# Function call argument (semantic difference)
f(x <- 1)
f(x <- 1, y <- 2)
x[i <- 1]
x[[i <- 1]]

# Parameter default (semantic difference)
function(a = b <- 1) a

# Control flow (syntax error)
if (x <- f()) y
while (x <- f()) y
for (i in x <- xs) y

# But a surrounding pair of parentheses lifts the restriction
if ((x <- f())) y
while ((x <- f())) y
for (i in (x <- xs)) y

# Tricky recursive cases with binary expression parents

# `?` parent and `<-` RHS
# `<-` to `=` would result in parse errors
f(a ? b <- 1)
x[a ? b <- 1]
x[[a ? b <- 1]]
if (a ? b <- 1) c
while (a ? b <- 1) c
function(x = a ? b <- 1) x

# `?` parent and `<-` LHS
# `<-` to `=` would result in semantic changes for `f()`, `x[]`, and `x[[]]`
# `<-` to `=` would result in parse errors for `if`, `while`, and `function`
f(a <- 1 ? b)
x[a <- 1 ? b]
x[[a <- 1 ? b]]
if (a <- 1 ? b) c
while (a <- 1 ? b) c
function(x = a <- 1 ? b) x

# `<-` parent
# `<-` to `=` would result in parse errors
f(a <- b <- 1)
x[a <- b <- 1]
x[[a <- b <- 1]]
if (a <- b <- 1) c
while (a <- b <- 1) c
function(x = a <- b <- 1) x
