for(x in xs) {
  x + 1
}

for(a_really_long_argument_name in but_we_dont_ever_break_inside_for_conditions_no_matter_how_long) {
  a_really_long_argument_name
}

# ------------------------------------------------------------------------------
# Autobracing

for(x in xs) {}
 
# Unconditional autobracing on for loop bodies to maximize clarity and intent
for(x in xs) x
for(x in xs) x + y

# ------------------------------------------------------------------------------
# Comments

for # leads for loop
(i in 1:5) {}

for ( # leads for loop
i in 1:5) {}

for (i # leads for loop
in 1:5) {}

for (i in # leads for loop
1:5) {}

for (i in
# leads for loop
1:5) {}

for (i in 1:5 # leads for loop
) {}

for (i in 1:5) # dangles {}
  {
  }

for (i in 1:5) # leads a
{
  a
}

for (i in 1:5) { # leads a
  a
}

for (i in 1:5) i # trails whole for loop

for (i in 1:5) { i } # trails whole for loop

# Comments 1-3 lead the whole for loop
# Comments 4-5 move to lead the body
for (
    # comment1
    # comment2
    a in 1
    # comment3
  ) # comment4
  # comment5
  {
    # comment6
    a
  }