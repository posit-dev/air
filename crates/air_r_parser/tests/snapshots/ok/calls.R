fn()
fn(a)
fn(a, b)
fn(...)
fn(a, ..., b)

fn(a = 1)
fn(a = )

fn(a = 1, b = 2)
fn(a = , b = 2)

fn(... = 1)
fn(... =)

fn(..1 = 2)
fn(..1 =)

fn("arg" = 1)
fn("arg" =)

# Comma tests
fn(,)
fn(,,,)
fn(,,a,,b,,)

# Comment tests
fn(
  # comment
)
fn(,
  # comment
,)
fn(
  a, # comment1
  b # comment2
)
fn(
  a # comment1
  = # comment2
  1 # comment3
)

{expr}(a = 1)

# `NULL` argument name (r-lib/tree-sitter-r#164)
fn(
  NULL = 1
)
fn(
  NULL = 
)
fn(
  NULL = ,
)