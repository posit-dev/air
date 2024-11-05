repeat 1

repeat {}

repeat { # a comment
}

repeat { # comment1
  # comment2
  1 + 1
}

# TODO: `comment1` should go inside the `{`
repeat # comment1
{
  # comment2
  1 + 1
}
