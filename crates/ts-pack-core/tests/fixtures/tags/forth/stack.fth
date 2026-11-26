\ Simple stack utilities
10 constant LIMIT
variable counter
0 value threshold
create buffer 64 allot

: square ( n -- n*n ) dup * ;
: cube ( n -- n^3 ) dup square * ;
: bump ( -- ) counter @ 1+ counter ! ;
: over-limit? ( n -- f ) threshold LIMIT max > ;

5 cube .
7 over-limit? .
