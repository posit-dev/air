## Lambda function

```r
a <- \(arg) arg
b <- \(arg1, arg2) paste0(arg1, arg2)
c <- \(fun, ...) fun(...)
1:3 |> {\(x, y = 1) x + y}() |> {\(x) sum(x)}()
{\(a = 1) a + 1}()
\() 1 + 2
\()

  1 + 2

# Not currently allowed by R's parser, but we think it will be and is just an oversight.
# `'\\'` would need to be included alongside `FUNCTION` here https://github.com/wch/r-source/blob/802121c877837926a6bc2a930b3da749b537258b/src/main/gram.y#L3898-L3901.
\
() 1 + 1
```
