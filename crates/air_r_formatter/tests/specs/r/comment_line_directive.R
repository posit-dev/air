#line 1 "top.R"
x <- 1

{
#line 1 "file1.R"
  withAutoprint({ show.srcref() })
}

f <- function() {
  x
#line 10 "gen.R"
  y
}

fn(
  a,
#line 20 "gen.R"
  b
)

{
  # Comments that only look like one stay indented
#lines of text
#line up the columns
  z
}

x #line 30 "gen.R"
