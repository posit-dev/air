## Identifiers

```r
foo
foo2
foo.bar
.foo.bar
.__NAMESPACE__.
.
..
._
._.
foo_bar
`_foo`
`a "literal"`
`another
literal \` foo`
`backslash followed by newline \
`
`\``
# Pipe placeholder
_
# Recognized as a single `_foo` identifier, even if invalid R code (r-lib/tree-sitter-r#71).
_foo
__foo
_foo_
```

## Unicode identifiers

```r
你好
.你.好.
.你_好.
```

## Invalid identifiers [error]

```r
# R does not allow identifiers to start with a `.` followed by a digit, as this
# would cause ambiguity with `.1L` and `.1i`. R itself rejects this as a syntax
# error, we currently parse this as `.1` followed by `foo`, just like we do with
# `1foo` (r-lib/tree-sitter-r#190).
.1foo
```

## Dot dot i

```r
..1
..10
```

## Unclosed backtick [error]

```r
`\`
```

## Dots

```r
# Parses, but fails at runtime
...

{ ... }

# An actual base R function
...()
```

## Dot dot i in functions and calls

```r
function(...) {
  ..1 + ..2
}

# Parses as a function
..1()
```
