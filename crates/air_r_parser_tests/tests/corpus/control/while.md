## While

```r
while(TRUE)
  bar

while(x > 0)
  x <- x - 1

while(TRUE)
  break

while(TRUE)
  next
```

## While no body [error]

```r
while (a < b)
```

## While with braces

```r
while(TRUE) FALSE

while(TRUE) {}

while(a) {

}

while(fn()) {

}
```

## A `while` with no condition [error]

```r
while () b
```
