repeat {}

repeat {
  a + b
}

repeat 
{
  a + b  
}

# ------------------------------------------------------------------------------
# Autobracing

repeat 1

repeat
  1 + 1

# ------------------------------------------------------------------------------
# Comments

repeat { # dangles {}
}

# These should be consistent
repeat { # leads a
  # leads a 2
  a
}
repeat # leads a
{
  # leads a 2
  a
}

repeat # dangles {}
{}

repeat # dangles {}
{
  # dangles {} 2
}

repeat
# leads a
{
  a
}

# leads repeat
repeat
{
  # leads a
  a
}

repeat # leads a
  a
