% family relations
:- dynamic parent/2.

parent(tom, bob).
parent(bob, ann).

male(tom).

grandparent(X, Z) :-
    parent(X, Y),
    parent(Y, Z).

ancestor(X, Y) :- parent(X, Y).
ancestor(X, Y) :-
    parent(X, Z),
    ancestor(Z, Y).

main :-
    ancestor(tom, ann),
    write(yes).
