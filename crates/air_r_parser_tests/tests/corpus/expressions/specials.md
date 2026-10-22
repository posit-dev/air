## Specials

```r
x %% y
x %/% y
x %+% y
x %>% y
x %>% 2 %>% z
x %some text% y
x %//% y
```

## A backslash inside a special is ordinary text

R's `SpecialValue` excludes only the newline, so the whole operator is one token.

```r
x %\% y
```

## An unterminated special [error]

The `%%` closes first, leaving a `%` with no partner before the newline.

```r
x %%% y
```

## Pipe

```r
x |> print()

x |> foo() %>% bar() |> baz()

x |> foo() |> bar() + baz()

x |> {function(x) x}()
```

## Pipe placeholder

```r
foo |> bar(x, y = _)
foo |> bar() |> baz(data = _)
```
