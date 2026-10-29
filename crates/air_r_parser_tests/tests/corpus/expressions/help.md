## Help operator

```r
a ? b
a ? b <- 1
?a
```

## Unary help doesn't absorb binary help

```r
?x ? y
?x ? y ? z
a ? ?b ? c
?function(x) x ? y
```

## Unary help absorbs assignment

```r
?x = 1
?x <- 1
```
