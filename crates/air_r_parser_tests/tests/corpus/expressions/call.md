## Calls

```r
f()
f(,)
f(,,)
f(x,)
f(x,,)
f(,x)
f(,,x)
f(x,,y)
f(,,x,,y,,)
f(x=,)
f("x"=,)
f(,y=)

f(x)
f(1+1)
f(1 ~ 1)
f(x, y)
f(x, y = 2)
f(x = 1 + 1)
f(x, y =)
f(f2(x, y))

# Dots as unnamed and named argument
f(...)
f(, ..., 1)
f(... = 1)
f(... = ,)
f(... = ...)

# `..i` as unnamed and named argument
f(..1)
f(, ..1, 1)
f(..1 = 1)
f(..1 = ,)
f(..1 = ..1)
```

## Calls with NULL as argument name

```r
fn(NULL =)
fn(NULL =,)
fn(NULL = 1)

# This was likely added to R to support this case for `switch()`
switch(typeof(x), NULL = this, integer = that)
```

## Calls with `return` as argument name

```r
# `return` is not a reserved word in R (it's not in `?Reserved`), so it is
# allowed as an argument name (r-lib/tree-sitter-r#189).

fn(return = TRUE)
fn(return = )

# One specific example from r-lib/tree-sitter-r#189
switch(
  x,
  return = ,
)
```

## `return` is just an identifier

```r
# `return` is a regular symbol in R, not a keyword (r-lib/tree-sitter-r#189). A `return()`
# call is parsed the same as any other call, with `return` as the
# identifier. A bare `return` is also parsed as an identifier.

function(x) return(x)
return(x)
return
```

## Switch

```r
switch(foo,
  x = 1,
  "y" = 2,
  z = ,
  3
)
```

## Calls with NA as argument name [error]

```r
# Unlike `NULL`, other keywords like `NA` aren't allowed here
fn(NA = )
```

## Not a call, subset, or subset2 [error]

```r
f
(x)

foo
[bar]

foo
[[x]]
```

## Not a call, subset, or subset2 due to sequential arguments [error]

```r
f(x y)
foo[x y]
foo[[x y]]

f(x, y z)
foo[x, y z]
foo[[x, y z]]

f(, x y)
foo[, x y]
foo[[, x y]]
```

## Call arguments

```r
fn()
fn(a)
fn(a, b)
fn(...)
fn(a, ..., b)

fn(a = 1)
fn(a = )

fn(a = 1, b = 2)
fn(a = , b = 2)

fn(... = 1)
fn(... =)

fn(..1 = 2)
fn(..1 =)

fn("arg" = 1)
fn("arg" =)

# Comma tests
fn(,)
fn(,,,)
fn(,,a,,b,,)

# Comment tests
fn(
  # comment
)
fn(,
  # comment
,)
fn(
  a, # comment1
  b # comment2
)
fn(
  a # comment1
  = # comment2
  1 # comment3
)

{expr}(a = 1)

# `NULL` argument name (r-lib/tree-sitter-r#164)
fn(
  NULL = 1
)
fn(
  NULL = 
)
fn(
  NULL = ,
)

# A standard call, not a special node (r-lib/tree-sitter-r#197)
return()
```

## Side by side arguments [error]

```r
fn(a b)
fn(a b c)
```

## An argument value that starts no expression [error]

Diagnostic should be about the faulty expression, not a comma

```r
fn(* 1)
fn(a, * 1)
fn(a = * 1)
fn(,, * 1)
```

Same for `[` and `[[`

```r
x[* 1]
x[[* 1]]
```

## Closing delimiter belonging to someone else [error]

The argument list ends at any closing delimiter, not just its own, so the `}`
survives for its owner rather than being eaten as an argument. Nothing was
written between the `(` and the `}`, so the list is empty rather than holding
one empty argument.

```r
fn(}
```

```r
fn(a}
```

## A newline or comment between a name and its `=`

`parse_argument()` looks one token past the name to find the `=`, and that
lookahead skips trivia to reach it

```r
f(a
  = 1)

f(a # c
  = 1)

f("a" # c
  = 1)
```
