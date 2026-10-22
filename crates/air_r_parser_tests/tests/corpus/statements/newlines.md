## Newlines

```r
apple
(banana)

{
  apple
  (banana)
}

(
  apple
  (banana)
)
```

## Two expressions on one line [error]

```r
1 2
```

## Two expressions on one line inside braces [error]

```r
{ 1 2 }
```

## A comment does not hide a line break

Lexer's `after_line_break` flag is sticky across trivia

```r
1 # c
2
```
