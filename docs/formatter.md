This document contains a running set of notes regarding important features or quirks that will eventually become a part of the Air documentation.

# Existing formatting

The Air formatter is guided by two competing principles:

- Existing formatting (including both whitespace and newlines) is ignored as much as possible. In an ideal world, there would be exactly "one true way" to format an R file. This avoids style discussions between collaborators, and allows them to focus their attention on the meaningful parts of the code.

- User preference of expanded or folded layouts should be respected. This is particularly noticeble with function calls:

  ```r
  # Folded onto one line
  list(a = 1, b = "two", c = 3)

  # Expanded over multiple lines
  list(
    a = 1,
    b = "two",
    c = 3
  )
  ```

  and with pipe chains:

  ```r
  # Folded onto one line
  df |> mutate(x = y + z) |> summarise(mean = mean(x))

  # Expanded over multiple lines
  df |>
    mutate(x = y + z) |>
    summarise(mean = mean(x))
  ```

  Expanded layouts can often be more readable, even when the folded layout fits on a single line.

Note that if Air unconditionally follows the first principle, then it would aggressively fold these examples onto one line because they'd fit within the line length.

For the most part, Air ignores existing formatting and makes formatting decisions based on whether or not a piece of code fits on a single line. However, there are important exceptions that allow Air to strike the right balance between these two competing principles.

## Empty lines

Empty lines allow you to provide some "breathing room" between expressions in your code, and are encouraged to improve readability. They are an example of a case where existing user formatting (here, multiple sequential newlines) is taken into account.

### Empty lines between expressions

Rather than stripping the empty lines between these expressions:

```r
# Input
1 + 1

2 + 2


3 + 3
```

and formatting them as:

```r
# Theoretical output
1 + 1
2 + 2
3 + 3
```

> Air respects up to 1 empty line between expressions.

That results in the following:

```r
# Output
1 + 1

2 + 2

3 + 3
```

### Empty lines between call arguments

> Within a function call, Air respects up to 1 empty line between arguments.

```r
fn(
  x = function() {
    y
  },

  # This is a really important argument that deserves its own comment,
  # the empty line above this is retained
  y = function() {
    y
  }
)
```

This is quite common in Shiny and in R6, where arguments are complex functions and often you do separate them with a full empty line, typically with a comment (or with roxygen2 documentation in the R6 case).

Leading empty lines before the first argument and trailing empty lines after the last argument are removed.

```r
# Input
fn(

  x = function() {
    y
  },

  y = function() {
    y
  }

)

# Output
fn(
  x = function() {
    y
  },

  y = function() {
    y
  }
)
```

### Empty lines within pipe chains

> Within a pipe chain, Air respects up to 1 empty line between the `operator` in the chain (like `|>`) and its immediate `right` hand side.

Empty lines like these are commonly seen in data analysis scripts, and are often accompanied with a leading comment.

```r
# Input
df |>
  foo() |>

  # Some extremely important comments about this complex call.
  # It's so important that we have multiple lines of comments about it.
  bar(
    option = "something_really_complex",
    option2 = 1 + trust_me_its_complicated()
  ) |>


  # Some more very important comments
  baz()

# Output
df |>
  foo() |>

  # Some extremely important comments about this complex call.
  # It's so important that we have multiple lines of comments about it.
  bar(
    option = "something_really_complex",
    option2 = 1 + trust_me_its_complicated()
  ) |>

  # Some more very important comments
  baz()
```

## Line breaks

Typically, Air aggressively removes line breaks in favor of placing as much as possible on a single line while still respecting the line width. After all, that's one of the main purposes of a formatter!

```r
# Input
for (
  x in xs
)
{
  x + 1
}

# Output
for (x in xs) {
  x + 1
}
```

However, there are a few places where it is much tougher to decide how much Air can fold without destroying user intent. These cases are documented below.

### Function calls

Consider the following data dictionary:

```r
# Input
dictionary <- list(
  bob = "burger",
  dina = "dairy",
  john = "juice"
)
```

This fits on one line! Shouldn't we fold it to this?

```r
# Theoretical output
dictionary <- list(bob = "burger", dina = "dairy", john = "juice")
```

In theory, yes. In practice, since this is a data dictionary it is typically perceived as more readable in the original fully expanded form. So should we always expand function calls? Well, note that _syntactically_, the `list()` call from above is quite similar to this function call:

```r
out <- fn(x = x, option1 = option1, option2 = option2)
```

And in that case many people would prefer this folded output to stay as is. Because there is no syntactic information to help Air decide between the two different styles, Air instead falls back to a heuristic based on the existing formatting:

> For function calls, if there is a line break between the `(` and the first argument, then the function call will be fully expanded.

Following that rule, this stays as is:

```r
# Input
dictionary <- list(
  bob = "burger",
  dina = "dairy",
  john = "juice"
)

# Output
dictionary <- list(
  bob = "burger",
  dina = "dairy",
  john = "juice"
)
```

To request that the dictionary be folded if possible, remove the leading line break, and run Air:

```r
# Input
dictionary <- list(bob = "burger",
  dina = "dairy",
  john = "juice"
)

# Output
dictionary <- list(bob = "burger", dina = "dairy", john = "juice")
```

To expand it back out, add the line break back, and run Air:

```r
# Input
dictionary <- list(
  bob = "burger", dina = "dairy", john = "juice")

# Output
dictionary <- list(
  bob = "burger",
  dina = "dairy",
  john = "juice"
)
```

Alternatively, you can use [codegrip](https://github.com/lionel-/codegrip) to explicitly swap between expanded and folded forms, and Air will respect that as long as the function call fits within the line width.

```r
# Input (codegrip)
dictionary <- list(bob = "burger", dina <cursor>= "dairy", john = "juice")

# Output (codegrip)
dictionary <- list(
  bob = "burger",
  dina = "dairy",
  john = "juice"
)
```

### Function definitions

> For function definitions, if there is a line break between the `(` and the first parameter, then the function definition will be fully expanded.

This function definition fits on one line without any issues:

```r
fn <- function(x, y, option = NULL, option2 = c("a", "b", "c")) {
  body
}
```

But it's reasonable to expand this out over multiple lines for readability, especially if you think that you may add more arguments in the future. Placing a line break after the `(` is a request for expanded form:

```r
# Input
fn <- function(
  x, y, option = NULL, option2 = c("a", "b", "c")) {
  body
}

# Output
fn <- function(
  x,
  y,
  option = NULL,
  option2 = c("a", "b", "c")
) {
  body
}
```

If you later decide to remove the optional arguments, the expanded form will remain untouched (due to the explicit line break after the `(`).

```r
# Input
fn <- function(
  x,
  y
) {
  body
}

# Output
fn <- function(
  x,
  y
) {
  body
}
```

To again request the folded form, remove the line break after the `(`:

```r
# Input
fn <- function(x,
  y
) {
  body
}

# Output
fn <- function(x, y) {
  body
}
```

Alternatively, you can use [codegrip](https://github.com/lionel-/codegrip) to explicitly swap between expanded and folded forms, and Air will respect that as long as the function signature fits within the line width.

```r
# Input (codegrip)
fn <- function(
  x,<cursor>
  y
) {
  body
}

# Output (codegrip)
fn <- function(x, y) {
  body
}
```

### Pipe chains

> For pipe chains and other binary operator chains, if there is a line break between the first `operator` in the chain and the immediate `right` hand side, then the pipe chain will be fully expanded.

This stays as is, even though it fits on one line, due to the existing line break after the first `|>`:

```r
# Input
df |>
  mutate(y = x + 1, z = x + 2) |>
  filter(x != y)

# Output
df |>
  mutate(y = x + 1, z = x + 2) |>
  filter(x != y)
```

Removing the line break after the first `|>` is a request to fold if possible:

```r
# Input
df |> mutate(y = x + 1, z = x + 2) |>
  filter(x != y)

# Output
df |> mutate(y = x + 1, z = x + 2) |> filter(x != y)
```

Note that pipe chain expansion is affected by function call expansion. If any function call in the pipe chain is expanded, then the pipe chain itself is forced to expand. In the following example, the newline after `mutate(` is a request for function call expansion, which forces the entire pipe chain to expand:

```r
# Input
df |> mutate(
  y = x + 1, z = x + 2) |> filter(x != y)

# Output
df |>
  mutate(
    y = x + 1,
    z = x + 2
  ) |>
  filter(x != y)
```

### Left assignment

> For left assignment of any kind (`<-`, `=`, and `<<-`), if there is a line break between the `operator` and the `right` hand side, then at most 1 line break will be retained.

Typically you'll see assignment in pipe chains like this, where the name of the input to the pipe chain is on the same line as the assignment operation.

```r
iris_long <- iris |>
  gather(measure, value, -Species) |>
  arrange(-value)
```

However, it is also acceptable to place a line break after the `<-`, which is a request to retain that line break.

```r
iris_long <-
  iris |>
  gather(measure, value, -Species) |>
  arrange(-value)
```

### Reversibility

Respecting existing line breaks in function calls, function definitions, pipe chains, and assignment gives the user a bit more power over how the final result of Air formatting looks. In these particular cases where there is no syntactic information Air can use, this is generally a good thing, as that extra bit of information can improve code readability.

However, respecting existing formatting does have some major drawbacks. Consider the following list:

```r
object <- list(important = 5, variable = "text", name = "andrew")
```

This list happily fits on a single line. Now consider what happens if we add one more field to the list

```r
# Input
object <- list(important = 5, variable = "text", name = "andrew", team = "panthers")

# Output
object <- list(
  important = 5,
  variable = "text",
  name = "andrew",
  team = "panthers"
)
```

This list now exceeds the line width, and is automatically split over multiple lines. Looks great! But what if we decide that `team` doesn't belong after all?

```r
# Input
object <- list(
  important = 5,
  variable = "text",
  name = "andrew"
)

# Output
object <- list(
  important = 5,
  variable = "text",
  name = "andrew"
)
```

We are now "stuck" with the expanded form even though it fits on one line, due to the explicit line break after the `list(`, which looks to Air like the user requested a line break (removing that line break recovers the folded form, but it requires an explicit action from the user). This is known as _irreversible_ formatting, and is something that Air generally tends to avoid where possible.

That might not seem like a huge deal, but if these changes all happened within the span of 1 commit, then you'll end up with an extraneous git diff:

```diff
-object <- list(important = 5, variable = "text", name = "andrew")
+object <- list(
+  important = 5,
+  variable = "text",
+  name = "andrew"
+)
```

This is unfortunate. Air thinks that both forms are valid, so it changes neither, but these kinds of nonsense git diffs are exactly what Air is supposed to help you avoid.

It also leaves the door open to [bikeshedding](https://en.wikipedia.org/wiki/Law_of_triviality), i.e. arguing over trivial issues, where your PR reviewer might have a _preference_ that you fold the list, but you have a _preference_ for the expanded form. Ideally, Air helps you avoid these conflicts entirely by enforcing one form or the other, but since both are valid to Air, it can't help.

Note that while the above example demonstrates this issue with function calls, it also holds true for function definitions, pipe chains, and assignment as well - i.e. any place where existing formatting is respected.

While respecting existing line breaks for these very specific cases is _generally_ desired to improve readability, we also recognize that some teams might want Air to be a fully reversible formatter - removing the possibility of erroneous diffs and bikeshedding entirely. We agree, as we think this is a great feature of a formatter, so this is one of the few places where we've provided an option. Supplying `--ignore-line-breaks` forces Air to completely ignore formatting related to existing line breaks.

# Tabs vs spaces

Air supports both tabs and spaces as possible indent styles, and supports specifying the number of spaces to use to represent an indent with when spaces is chosen as the indent style.

We believe that tabs are a better choice because:

- They allow per-person customization rather than per-project customization

- As a consequence of the above, they are much better for accessibility

Because of this, Air defaults to using tabs rather than spaces. See below for a detailed explanation of why.

## Per-person customization

IDEs provide user level options that allow you to customize how "wide" a single tab is (i.e. how many virtual spaces it is shown with). For example, a tab-width of 2 looks like:

```r
list(
  a = 1,
  banana = "yellow",
  this = that
)
```

And a tab-width of 4 looks like:

```r
list(
    a = 1,
    banana = "yellow",
    this = that
)
```

Note that this is a _user-level_ preference. Under the hood, that whitespace is just represented by a single `\t` on disk. If you prefer a tab-width of 2 spaces, then your collaborator is still free to choose a tab-width of 4 spaces. The actual contents of the file you two are looking at are exactly the same.

Spaces do not allow that level of user customizability. They require all collaborators to use the same _project-level_ preference of, say, 2 spaces for a single indent. This removes a degree of customization, which is particularly important for accessibility (see below).

In VS Code and Positron, tab-width corresponds to the setting `editor.tabSize`, with a default of `4`. Also note that there is a `editor.detectIndentation` setting which can automatically detect if a file uses tabs or spaces for indentation.

## Tabs for accessibility

Because a single tab unconditionally represents one level of indentation, they are a great asset for programmars with both partial and full visual impairment:

- Tabs make it easier for screen readers and braille displays to correctly identify indentation level, and do so very compactly, which can be important for braille displays with a limited number of braille cells.

- Tab-width can be set extra wide, which is beneficial when paired with a wide monitor.

- Tab-width can be set extra narrow, which is beneficial when paired with an extra large font size.

In general, we embrace tabs for their customizability and unambiguous meaning. This helps programmars read code comfortably using tools and settings that are appropriate for them.

Some additional discussions of this topic:

- https://adamtuttle.codes/blog/2021/tabs-vs-spaces-its-an-accessibility-issue

- https://www.reddit.com/r/javascript/comments/c8drjo/nobody_talks_about_the_real_reason_to_use_tabs

- https://www.reddit.com/r/programming/comments/voirfg/default_to_tabs_instead_of_spaces_for_an/

## Mixing tabs and spaces

The motivation of providing an accessible friendly formatter has informed some choices regarding how Air formats code. In particular, Air does not support the "hanging" style of indents because it requires a mix of tabs (for indents) and spaces (for extra alignment) by design. Consider the following:

```r
fn <- function(x,
               y,
               option = NULL,
               extra = c("a", "b")) {
    out <- list(x,
                y)
}
```

To implement hanging indent style, you align all arguments after the opening `(`. In this case, `fn <- function(` takes up 15 characters, so there are 15 spaces before `y`, `option`, and `extra`.

To implement this with tabs, the formatter would first need to know about the tab-width that the user's IDE is using. Say a tab-width of 4 was being used by the author of this code, then the formatter would attempt to insert 3 tabs  followed by 3 spaces to retain the visual appearance of the above code, and that would be saved to disk (`3 tabs * 4 tab-width + 3 spaces = 15 spaces`).

Now say a collaborator opens this file in their IDE, where they have tab-width set to 2. That results in this visual representation of the code:

```r
fn <- function(x,
         y,
         option = NULL,
         extra = c("a", "b")) {
  out <- list(x,
        y)
}
```

In the collaborator's case, their IDE visually represents the 3 tabs and 3 spaces as 9 total spaces (`3 tabs * 2 tab-width + 3 spaces = 9 spaces`), which doesn't align with the opening line of the `function(` anymore. Notice this has also affected the function call to `list()`, due to the same principle.

The key insight here is that the only time this can be an issue is when tabs are used for a base level of indentation followed by an extra series of spaces used for alignment. A formatter can help avoid this entirely by never printing code that results in this mixed style.

## Line width and tab width

There is one slightly confusing element to tab-width related to line-width that's worth mentioning. Consider the following:

```r
fn <- function() {                               # 50 character line width
  my_cool_function(with_one_arg_here, and_there) #
}                                                #
```

In this example, assume a 50 character line width is being used, which has been marked with the comment. Specifically, the settings are:

```
# Air settings (project level)
LineWidth = 50
IndentStyle = Tab
IndentWidth = 2

# IDE settings (user level)
tab-width = 2
```

One nice feature of Air is that when code exceeds the line width, it is automatically expanded over multiple lines. But when tabs are used as leading indentation, how many characters does a tab represent? To determine that, Air looks to the `IndentWidth`, which is otherwise only useful when `IndentStyle` is `Space`. The indent width of 2 plus the 46 characters in the call to `my_cool_function()` puts us at 48 characters - i.e. less than the line length, so it stays folded. Note that the `tab-width` setting of the IDE is also 2. This means that if you had added a vertical "ruler" in your IDE at 50 characters (here, represented by the `#` characters), then everything would look normal.

But imagine if your collaborator sets their `tab-width` to 8. In that case, it would visually look like the line takes up 54 (8 + 46) characters, but to Air it still only takes up 48 characters:

```r
fn <- function() {                               # 50 character line width
        my_cool_function(with_one_arg_here, and_there)
}                                                #
```

If they also had a vertical ruler set at 50 characters, then this might look strange to them because they expected a line break after 50. If the project wide Air `IndentWidth` was changed to 8, then everything would look normal for the collaborator, but for the user with tab-width of 2 lines would start breaking earlier than expected.

This usage of `IndentWidth` when using an `IndentStyle` of `Tab` is the one place in Air where tab-width (typically a user-level setting) leaks over into project-level setting space. That said, the worst thing that can happen is that lines look a little shorter or longer than you may have expected.

# Function definition styles

In Air, there are two accepted forms for function definitions - folded and expanded.

```r
# Folded
fn <- function(x, y, option = NULL, extra = c("a", "b")) {
  body
}

# Expanded
fn <- function(
  x,
  y,
  option = NULL,
  extra = c("a", "b")
) {
  body
}
```

When you exceed the line width, Air will automatically switch from folded to expanded. You can also manually request the expanded form by inserting a line break after the opening `(` in `function(`. See [line breaks with function definitions](#function-definitions) for more examples of this.

See the section on [tabs vs spaces](#mixing-tabs-and-spaces) for why Air does not allow hanging indent function definitions.
