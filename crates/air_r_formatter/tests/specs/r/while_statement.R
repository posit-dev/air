while(a){}

while(a) {
  1 + 1
}

while({ complex }) {
  1 + 1
}

while(super_long_function_name_is_true_man_this_is_a_really_really_long_function()) {
  1 + 1
}

# ------------------------------------------------------------------------------
# Autobracing

while(a)a

while(a) 
  a

# ------------------------------------------------------------------------------
# Comments

while # leads while
(a) {
  b
}

while
# leads while
(a) {
  b
}

while(a # leads while
) {
  b
}

while(a
# leads while
) {
  b
}

while( # leads while
  a) {
  b
}

while(
  # leads while
  a) {
  b
}

while(
  a
  # leads while
) {
  b
}

while(a) # leads b
{
  b
}

while(a)
# leads b
{
  b
}

while(a) # dangles {}
{}

while(a) # leads b
b

while(a) # dangles {}
{
  # dangles {} 2
}
