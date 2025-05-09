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

for # comment
(i in 1:5) {
}

for ( # comment
i in 1:5) {
}

for (i # comment
in 1:5) {
}

for (i in # comment
1:5) {
}

for (i in
# comment
1:5) {
}

for (i in 1:5 # comment
) {
}

for (i in 1:5) # comment
  {
  }

for (i in 1:5) i # comment

for (i in 1:5) { i } # comment

# All comments enclosed by the for statement get lifted up
for (
    # comment1
    # comment2
    a in 1
  ) # comment3
  # comment4
  {
    # comment5
    a
  }