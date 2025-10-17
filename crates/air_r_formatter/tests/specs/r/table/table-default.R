# Should format as table by default
tribble(
~x,~y,
1,2,
3,4
)

a <- tribble(
~x,~y,
1,2,
3,4
)

fcase(
x<5L,1L,
x>5L,3L,
default=5L
)

a = fcase(
x<5L,1L,
x>5L,3L,
default=5L
)

# "Sees through" namespaces and looks at the function name
tibble::tribble(
~x,~y,
1,2,
3,4
)
