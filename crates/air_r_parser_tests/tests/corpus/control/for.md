## For

```r
for (x in y)
  f

for (x in 5:6) {
  for (y in ys) {
    z
  }
}

for (x in y) for (y in z) x + y

for (x in y

) 1 + 1
```

## For no body [error]

```r
for (i in 1:5)
```

## For with comments

```r
for (i in 1) i

for (
  i # comment
  in # comment2
  1 # comment3
) i

for # dangling1
  ( # dangling2
    a in 1
  ) # dangling3
  a
```

## `...` and `..i` as the `for` variable

Technically allowed!

```r
for (... in 1:2) print(get("..."))
for (..1 in 1:2) print(get("..1"))
```

## A `for` variable that is not a name [error]

```r
for (1 in x) y
```

## A `for` with no variable [error]

```r
for (in x) y
```

## A `for` with no sequence [error]

```r
for (i in) y
```
