\ Simple stack utilities
10 constant LIMIT
variable counter
create buffer 64 allot

: square ( n -- n*n ) dup * ;
: cube ( n -- n^3 ) dup square * ;
: bump ( -- ) counter @ 1+ counter ! ;

5 cube .
