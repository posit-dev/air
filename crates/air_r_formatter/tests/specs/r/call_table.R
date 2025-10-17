# fmt: table
list(
)

# fmt: table
list(1+1)

# One argument
# fmt: table
list(
1+1
)

# One row
# fmt: table
list(
1+1, 1+1
)

# Incomplete table
# fmt: table
list(
1+1, 1+10, 1+100,
1+1,
)

# Jagged table
# fmt: table
list(
1+10, 1+100,
1+1,
1+1, 1+1000, 1+1,
)

# Table
# fmt: table
list(
1+1, 1+10,
1+100, 1+1000,
)

# Indented
{
# fmt: table
list(
1+1, 1+10,
1+100, 1+1000,
)
}

# Typical usage
# fmt: table
tribble(
~quarter,~region,~product,~price,~units_sold,
"Q1","NorthWest","Laptop",1499.99,250,
"Q2","South","Laptop",489.5,196,
"Q1","South","Tablet",249.99,304,
"Q1","NorthWest","Tablet",259.95,340,
)

# fmt: table
standardized <- tribble(
~from,~to,
c("UNC","Chapel Hill"),"UNC",
c("Duke","Duke University"),"Duke",
c("NC State"),"NC State",
c("ECU","East Carolina"),"ECU",
NA,NA
)

# skipped
# fmt: skip
# fmt: table
tribble(
1,1,
2,2
)

# fmt: table
# fmt: skip
# skipped
tribble(
1,1,
2,2
)

# Very long
# fmt: table
list(
foooooooooo,baaaaaaaaar,foooooooooo,baaaaaaaaar,foooooooooo,baaaaaaaaar,foooooooooo,baaaaaaaaar,foooooooooo(baaaaaaaaar,foooooooooo,baaaaaaaaar),
1+100, 1+1000,
)

# ------------------------------------------------------------------------
# Comments

# fmt: table
list(# comment1
# comment2
)# comment3

# fmt: table
list(1+1, 1+1 # comment
)

# fmt: table
list(
  1+1, 1+1, # comment1
  1+1, 1+1 # comment2
)

# fmt: table
list(
  # comment1
  1+1, 1+1,
  # comment3
  1+1, 1+1
)

# fmt: table
list(
  1,2    # comment
)

# Unfortunate: comment3 gets pulled up by Biome's Comments builder
# fmt: table
list(
  foo  # comment1
  =1, bar  # comment2
  =  # comment3
  2,
)

# ------------------------------------------------------------------------
# Holes

# fmt: table
list( , )

# fmt: table
list(
,,
,,
)

# fmt: table
list(
,
,,
)

# fmt: table
list(
,
,,,,
,,10,,
,"foo",
)


# ------------------------------------------------------------------------
# Commas

# fmt: table
list(
  1
)

# fmt: table
list(
  1 ,
)


# ------------------------------------------------------------------------
# Assignments

# fmt: table
foo <- list(
  1+1, 1+1,
  1+1, 1+1
)

# fmt: table
foo <<- list(
  1+1, 1+1,
  1+1, 1+1
)

# fmt: table
foo = list(
  1+1, 1+1,
  1+1, 1+1
)

# This is a syntax error, only base assignments are special-cased
# fmt: table
foo %=%
  list(
    1+1, 1+1,
    1+1, 1+1
  )

# Fallback syntax for these cases
foo %=%
  # fmt: table
  list(
    1+1, 1+1,
    1+1, 1+1
  )


# ------------------------------------------------------------------------
# Alignment

# Strings - should left-align
# fmt: table
list(
  "1",
  "12",
  "100",
  "3",
)

# Non-numeric types - should left-align
# fmt: table
list(
  "1",
  ho,
  foobar,
  f(),
)

# Integers - should right-align
# fmt: table
list(
  1L,
  12,
  100,
  3,
)

# Pure decimals - should align at decimal point
# fmt: table
list(
  0.,
  1.5,
  12.34,
  100.0,
  3.456,
)

# Mixed integers and decimals
# fmt: table
list(
  1000,
  2.5,
  50L,
  123.456,
  9,
  0.1,
  0.
)

# Mixed integers and decimals
# fmt: table
list(
  0.,
  1,
  2.
)

# Complex decimal alignment with varying precision
# fmt: table
list(
  1.1,
  22.22,
  333.333,
  4444.4444,
  55.55,
  6.6,
  777,
  8.8,
  99.99,
)

# Mixed types - should align depending on type
# The last argument doesn't have a comma.
# The mixed alignment is not great, but is consistent.
# fmt: table
list(
  1.10,
  "12",
  1,
  2L,
  "3",
  10000,
  3.5,
  f()
)

# ------------------------------------------------------------------------
# Unary operators

# fmt: table
list(
  -1.200,
  +2L,
  -100.
)

# With repeated unary operators we fall back to regular parsing. Note how the
# numeric arguments are not aligned with the number in the first row.
# fmt: table
list(
  --1.200,
  foo(),
  +-20L,
  0,
  -100.
)

# ------------------------------------------------------------------------
# Hard lines

# All of these fall back to verbatim because of the presence of hard lines in
# the source or the formatted output. Note that these tests cause Biome's
# snapshotter to add an "Unimplemented nodes/token" section below because of the
# verbatim fallback.

# fmt: table
list(
"foo
",  2)

# fmt: table
list(
{ foo },  2
)

# These line breaks are removed by the flat layout formatter
# fmt: table
list(
  c(
    1, 2
  ), 3,
  c(4, 5), 6,
  c(
    7, 8), 9
)

# ------------------------------------------------------------------------
# Named argument

# The first named argument stops the table. That argument and all
# that follow it are laid out in expanded form.

# Typical case
# fmt: table
fcase(
  x < 5L, 1L,
  x > 5L, 3L,
  default = 5L
)

# fmt: table
list(
  1,2,
  3,4,

  foo = 1, 2, 3
)

# fmt: table
list(
  1,2,
  3,4,

  # comment
  foo = 1, 2, 3
)
