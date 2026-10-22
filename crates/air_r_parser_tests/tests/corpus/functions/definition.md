## Functions

```r
function() 1
function() {}
function(arg1, arg2) {
  arg2
}

function(arg1, arg2 = 2) {}

function(x,
         y,
         z = 3) {

}

function()


  1

function
() 1 + 1

function

() 1 + 1

function

# this important comment

() 1 + 1

function() function() {}

function(x = function() {}) {}

# With no intermediate `{` scope
function() for(i in 1:5) i
```

## Function no body [error]

```r
function(x, y)
```

## Function no body with assignment [error]

```r
x <- function(x, y)
```

## Function no body inside another function [error]

```r
function(x = function()) {}
```

## Function definitions

```r
function(a) a
function(a, b) a + b
function(
  a # important!
) a
function(...) 1
function(a, ..., b) 1
function(a = 1, ..., b = 2) a
function(..1, ..2) get("..1") + get("..2")
function(..1 = 1, ..2 = 2) get("..1") + get("..2")
function(
  x = # important!
  4
) 1

\(x, y) 1
```

## A body stops at the binary `?`

Weirdly in R, help is precedence 1 and function and loop bodies are precedence 2.
So `function() x ? y` must parse as `(function () x) ? y`.

```r
function() a ? b
function() a ? b ? c
?function() a
```

```r
function() a <- b
function() a = b
1 + function(x) x + 2
```

## A definition with no parameter list [error]

```r
function 1
\ 1
function
```
