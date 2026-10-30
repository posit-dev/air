## Literal argument name [error]

```r
f(1 = "a")
```

## Binary expression argument name [error]

```r
f(x + 1 = 2)
```

## Call argument name [error]

```r
f(g(x) = 1)
```

## Unary expression argument name [error]

```r
f(-x = 1)
f(~x = 1)
f(!x = 1)
```

## Keyword argument name [error]

```r
f(TRUE = 1)
```

## Chained `=` in a named argument value [error]

```r
f(x = y = 1)
```

## `=` after `<-` in an argument [error]

```r
f(x <- y = 1)
```

## `=` after `->` in an argument [error]

```r
f(x -> y = 1)
```

## `=` after `?` in an argument [error]

```r
f(a ? b = c)
```

## `=` after `?` after a function in an argument [error]

```r
f(function(x) x ? y = 1)
```

## Subset argument [error]

```r
x[1 = 2]
```

## Subset2 argument [error]

```r
x[[1 = 2]]
```

## Chained `=` in a named subset argument value [error]

```r
x[a = b = 1]
```

## Parameter default [error]

```r
function(x = y = 1) x
```

## `if` condition [error]

```r
if (x = 1) 2
```

## `while` condition [error]

```r
while (x = 1) 2
```

## `for` sequence [error]

```r
for (i in x = 1) 2
```

## Recovery continues with the next argument [error]

```r
f(1 = "a", b)
```

## Recovery with chained `=` [error]

```r
f(1 = a = b)
```

## Recovery with a following `?` [error]

```r
f(1 = a ? b)
```

## Recovery continues with the next parameter [error]

```r
function(x = 1 = 2, y) x
```

## Recovery with a missing RHS [error]

```r
f(1 = )
```

## Parentheses and braces allow `=`

```r
f((1 = 2))
f({1 = 2})
x[(1 = 2)]
x[[(1 = 2)]]
if ((x = 1)) 2
while ((x = 1)) 2
for (i in (x = 1)) 2
function(x = (y = 1)) x
```

## Bodies allow `=`

```r
f(function(x) x = 1)
f(\(x) y = 1)
f(x = function(y) y = 1)
f(x <- function(y) y = 1)
f(function(x) x <- y = 1)
f(if (a) b = 1 else c = 2)
f(if (a) b else c = 2)
f(repeat x = 1)
f(while (a) x = 1)
f(for (i in x) y = 1)
function(x = function(y) y = 1) x
```

## Unary `?` allows `=`

```r
f(?x = 1)
f(?x = 1 = 2)
f(a ? ?b = 1)
```

## `?` is allowed

```r
f(a = b ? c)
f(a ? b)
if (a ? b) 1
```

## Other assignments are allowed

```r
f(x = y <- 1)
f(x <- 1)
f(x -> y)
if (x <- 1) 2
function(x = y <- 1) x
```
