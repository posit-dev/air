#' [format]
#' table = ["foo"]

foo(
~x,~y,
1,2,
3,4
)

a <- foo(
~x,~y,
1,2,
3,4
)

bar(
~x,~y,
1,2,
3,4
)

# Should format as table by default
tribble(
~x,~y,
1,2,
3,4
)

# Should format as table by default
fcase(
x<5L,1L,
x>5L,3L,
default=5L
)
