::  a small core with arms
|%
+$  count  @ud
++  add-one
  |=  n=@ud
  ^-  @ud
  +(n)
++  double
  |=  n=@ud
  (mul 2 n)
+$  pair  [p=@ud q=@ud]
++  swap
  |=  [a=@ud b=@ud]
  ^-  pair
  [b a]
++  quadruple
  |=  n=@ud
  (double (double n))
--
