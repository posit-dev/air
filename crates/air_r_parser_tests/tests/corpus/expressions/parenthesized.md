## Parenthesized expression

```r
(1)
((1))
(1 + 1)
(fn(a, b))
fn((a), (b))
(function() {
  body
})
```

## Not a parenthesized expression 1 [error]

```r
()
```

## Not a parenthesized expression 2 [error]

```r
(
  1
  2
)
```

## Not a parenthesized expression 3 [error]

```r
(1; 2)
```

## Parentheses with comments

```r
({
  1 + 1
})

( # comment1
  # comment2
  1 # comment3
  # comment4
) # comment5
```
