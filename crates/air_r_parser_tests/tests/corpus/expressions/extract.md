## Dollar

```r
foo$bar
foo$bar$baz
foo$bar@baz
foo$bar()
foo$"bar"
foo$bar()$baz[[1]]$bam
foo$...
foo$..1
```

## Slot

```r
foo@bar
foo@bar$baz
foo@bar()
foo@"bar"
foo@...
foo@..1
```

## Dollar, at, namespace, namespace internal with expression RHS [error]

```r
# The RHS of these nodes must be a string/identifier, so we nicely get a true
# node here alongside the braces. Even if that's not parsable R code, it's
# useful for completions and highlighting.
foo${bar}
foo@{bar}
foo::{bar}
foo:::{bar}
```

## Dollar no RHS [error]

```r
foo$
```

## Slot no RHS [error]

```r
foo@
```

## Dollar, at, namespace, namespace internal with `if` RHS [error]

```r
# The RHS's of these nodes are restricted to strings and identifiers, so the RHS
# is left empty and the `if` goes on to be its own statement, which then errors
# because it doesn't have a `(`. This is actually pretty decent behavior.
foo$if
foo@if
foo::if
foo:::if
```

## Dollar, at, namespace, namespace internal with newline before operator [error]

```r
# Newlines are not allowed before the operator, so none of these are valid
x
$y

x
@y

x
::y

x
:::y
```

## Extract operators

```r
a$b
a@b

a$`b`
a@`b`

a$"b"
a@"b"

a()$b
a()@b

a$b$c
a@b@c

a$b()
a@b()

a$...
a@...

a$..1
a@..1

a$..10
a@..10
```

## Extract with a literal RHS [error]

The RHS slot is left missing rather than swallowing the invalid right-hand side.

```r
a$1
a$NA
a$NULL
a$TRUE
a$(b)
a${ b }

a@1
a@NA
a@NULL
a@TRUE
a@(b)
a@{ b }
```

## Dollar tolerates a newline

```r
df$
  x
```
