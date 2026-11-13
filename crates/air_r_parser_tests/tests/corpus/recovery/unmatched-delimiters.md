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

## Opening bracket, cut off by the end of the file [error]

```r
x[1
```

## Opening parenthesis of a condition, cut off by the end of the file [error]

The diagnostic for the missing `)` points back at the `(` it should close.

```r
if (x
```

```r
while (x
```

```r
for (i in x
```

## Missing both parentheses of a condition [error]

With no `(` there is nothing to point back at, so the missing `)` is reported
on its own.

```r
if x y
```

```r
while x y
```

```r
for i in x y
```
