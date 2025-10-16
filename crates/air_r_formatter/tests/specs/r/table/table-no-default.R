#' [format]
#' table = ["foo"]
#' default-table = false

# Should not format as table
tribble(
~x,~y,
1,2,
3,4
)

# Should not format as table
fcase(
x<5L,1L,
x>5L,3L,
default=5L
)

foo(
~x,~y,
1,2,
3,4
)
