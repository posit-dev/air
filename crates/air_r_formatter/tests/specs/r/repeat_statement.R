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

repeat { # leads a
  # leads a part 2
  a
}

repeat # leads repeat
{
  # leads a
  a
}

repeat # leads repeat
{}

repeat # leads repeat
{
  # dangles {}
}

repeat
# leads repeat
{
  a
}

# leads repeat
repeat
{
  # leads a
  a
}

repeat # leads repeat
  1
