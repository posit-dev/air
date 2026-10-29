## Formulas

```r
~x
y~x
```

## Unary tilde doesn't absorb binary tilde

```r
~x ~ y
~ ~x ~ y
a ~ ~b ~ c
```

## Unary tilde absorbs tighter operators

```r
~x | y
```

## Unary tilde doesn't absorb assignment

```r
~x <- 1
~x -> y
```
