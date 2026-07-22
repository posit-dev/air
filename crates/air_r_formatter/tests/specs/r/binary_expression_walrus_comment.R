#| [format]
#| persistent-line-breaks = false

# https://github.com/posit-dev/air/issues/512
# An own-line comment after `:=` keeps the expression expanded and idempotent.
walrus :=
  # comment
  value

# A comment-free line break still follows persistent-line-breaks = false.
walrus_without_comment :=
  value

# An end-of-line comment still allows the walrus expression to flatten.
walrus_end_of_line := # comment
  value
