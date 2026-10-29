## Assignment

```r
x <- 1
x = 1
x := 1
x <<- 1
1 ->> x
1 -> x
x <- y(1)
y(1) -> x
```

## Equals assignment binds looser than left assignment

`x <- y = 1` is a runtime error in R, but does technically parse like `(x <- y) = 1`.
`x = y <- 1` parses and runs as `x = (y <- 1)`.

```r
x <- y = 1
x = y <- 1
```

## Equals assignment binds looser than right assignment

```r
x -> y = 1
x = y -> z
```

## Equals assignment binds tighter than help

```r
x = y ? z
x ? y = z
```
