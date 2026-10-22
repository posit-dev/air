## Functions with `return` as parameter name

```r
# `return` is not a reserved word in R (it's not in `?Reserved`), so it is
# allowed as a parameter name (r-lib/tree-sitter-r#189).

function(return) return
function(return = 1) return
```

## Function dots and dot dot i

```r
# Dots as parameter without default
function(...) { list(...) }

# Dots as parameter with default
function(... = 1) { get("...") }

# `..i` as parameter without default
function(..1) { get("..1") }

# `..i` as parameter with default
function(..1 = 1) { get("..1") }

# Miscellaneous
function(x, ...) f(...)
function(x, ...) ..1 + ..2
```

## A hole in a parameter list [error]

```r
function(a,,b) 1
function(a,) 1
function(,) 1
```

## A parameter that is not a name [error]

```r
function(1) x
function("a") x
function(a$b) x
```

Recovery stops at the `=`, so a bad name costs the name and nothing else.

```r
function(1 = 2) x
function("a" = 1) x
function(a, 1 = 2, b) x
function(= 1) x
```

## A missing comma between parameters [error]

A name is a single token, so the token after one may well be another parameter.
Reporting the separator without consuming anything keeps both potential parameters.

```r
function(a b) x
```

## A default with no value [error]

The default is required once an `=` is written (r-lib/tree-sitter-r#161).

```r
function(x = ) 1
```

## A parameter list with no closing parenthesis [error]

The list runs to the end of the file and the `)` is reported missing, leaving
`RParameters` and the definition around it whole.

```r
function(a
```

```r
\(x
```

A closer that belongs to someone else ends the list without being consumed

```r
function(a} 1
function(a] 1
```
