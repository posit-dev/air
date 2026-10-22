## Semicolons

```r
1;2

1 ; 2

{
    1;
}

{
    1; 2
}

1;# comment
1; # comment
```

## Semicolon at start of file [error]

```r
;
1
```

## Semicolon at end of file

```r
# posit-dev/air#238
1;
```

## Semicolon on its own line at end of file [error]

```r
1

;
```

## A semicolon with no expression is an error at top level [error]

```r
;
```

```r
;;
```

```r
;1
```

```r
1;;
```

```r
1;;2
```

```r
1
;
```

The comment has no expression to attach to, so it lands on the `;`.

```r
;# comment
```

## A semicolon with no expression is allowed inside braces

```r
{
  ;
}
```

```r
{
  ;;
}
```

```r
{
  ;1
}
```

```r
{
  1;;
}
```

```r
{
  1
  ;
}
```

```r
{
  1

  ;
}
```

```r
{
  ;# comment
}
```

## A semicolon terminates any kind of statement

```r
x <- 1;
f(1);
(1);
{1};
if (TRUE) 1;
for (i in 1) i;
while (FALSE) 1;
repeat break;
function() 1;
```
