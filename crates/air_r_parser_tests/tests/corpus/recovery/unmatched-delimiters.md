## Closing brace [error]

```r
}
```

## Closing parenthesis [error]

```r
)
```

## Closing bracket [error]

```r
]
```

## Opening brace, closing parenthesis [error]

```r
# Parenthesis is "not valid" so it isn't matched as a closer for the `{`
{)
```

## Opening parenthesis, closing brace [error]

```r
(}
```

## Opening parenthesis, closing bracket [error]

```r
(]
```

## Opening bracket2, unmatched closing bracket [error]

```r
x[[2]
```

## Opening bracket and bracket2, unmatched closing bracket [error]

```r
x[y[[2]
```

## Opening bracket2 and bracket, matched closing bracket [error]

```r
x[[y[2]
```

## Opening bracket2 and bracket, unmatched closing bracket [error]

```r
x[[y[2]]
```

## An argument name cut off by the end of the file [error]

```r
f(a
```

```r
f(a=
```
