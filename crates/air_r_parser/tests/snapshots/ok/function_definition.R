function() 1
function(a) a
function(a, b) a + b
function(
  a # important!
) a
function(...) 1
function(a, ..., b) 1
function(a = 1, ..., b = 2) a
function(..1, ..2) get("..1") + get("..2")
function(..1 = 1, ..2 = 2) get("..1") + get("..2")
function(
  x = # important!
  4
) 1

\(x, y) 1
