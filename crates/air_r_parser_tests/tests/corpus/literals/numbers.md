## Integers

```r
12332L
0L
12L
1e1L
1L
1e5L
# Technically, R parses these as floats with a warning, but for our purposes this is good enough
0.1L
# r-lib/tree-sitter-r#198
.1L
```

## Floats

```r
.66
.11
123.4123
.1234
x <- -.66
```

## Hexadecimal

```r
# `x` vs `X`
0x123
0X123

# Numbers and letters
0xDEAD
0XDEAD
0x1f2F3
0X1f2F3

# `p` vs `P`
0x0p0
0x0P0
0x0p123
0x0P123

# `+` and `-`
0x0p+0
0x0p-0
0x0p+123
0x0p-123

# Decimal point
0x.
0x1.
0x.1
0x1.1

# Decimal point with exponent
0x.p1
0x1.p1
0x.1p1
0x1.1p1

# As integers
0x123L
0X123L
0xDEADL
0XDEADL
0x1f2F3L
0X1f2F3L
0x0p0L
0x0P0L
0x0p123L
0x0P123L
0x0p+0L
0x0p-0L
0x0p+123L
0x0p-123L
0x.L
0x1.L
0x.1L
0x1.1L
0x.p1L
0x1.p1L
0x.1p1L
0x1.1p1L
```

## Scientific notation floats

```r
1e322
1e-3
1e+3
1.8e10
1.e10
1e10
```

## Complex

```r
1i
0i
1.1i
# r-lib/tree-sitter-r#198
.1i
1e10i
.1e10i
0x1i
0x.1i
```

## Double values

```r
1
2.5
1e6
0x123F

# r-lib/tree-sitter-r#198
.1
```

## Complex values

```r
2.5i
1e6i
0x123Fi
```
