## Namespace get

```r
foo::bar
foo::bar(1)
foo::...
foo::..1
...::foo
..1::foo
```

## Namespace get internal

```r
foo:::bar
foo:::bar(1)
foo:::...
foo:::..1
...:::foo
..1:::foo
```

## Namespace with missing RHS [error]

```r
# It's nice that `::` enforces that its RHS can only be a string or identifier,
# so this gives us a pretty clean tree even though it is invalid R code.
# r-lib/tree-sitter-r#65
library(dplyr::)

library()

1
2
3
```

## Namespace with missing RHS - 2 [error]

```r
foo::
```

```r
foo:::
```

## Namespace operators

```r
a::b
a:::b

a::`b`
a:::`b`

a::"b"
a:::"b"

a::b()
a:::b()

a::b$c()
a:::b@c()

a::...
a:::...

...::a
...:::a

...::...
...:::...

a::..1
a:::..1

..1::a
..1:::a

..1::..1
..1:::..1
```

## Namespace with a literal RHS [error]

The RHS slot is left missing rather than swallowing the invalid right-hand side.

```r
a::1
a::NA
a::NULL
a::TRUE
a::(b)
a::{ b }

a:::1
a:::NA
a:::NULL
a:::TRUE
a:::(b)
a:::{ b }
```

## Chained `::` [error]

```r
a::b::c
```

## Chained `:::` [error]

```r
a:::b:::c
```

## Call on the left of `::` [error]

```r
a()::b
```

## Call on the left of `:::` [error]

```r
a():::b
```

## Namespace newline is significant only outside parens

```r
f(stats::
  median)
```
