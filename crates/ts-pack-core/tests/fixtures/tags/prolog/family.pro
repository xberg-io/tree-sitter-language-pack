% family relations
:- module(family, [ancestor/2, siblings/2]).
:- use_module(library(lists)).
:- dynamic parent/2.

parent(tom, bob).
parent(tom, liz).
parent(bob, ann).

male(tom).
male(bob).

grandparent(X, Z) :-
    parent(X, Y),
    parent(Y, Z).

ancestor(X, Y) :- parent(X, Y).
ancestor(X, Y) :-
    parent(X, Z),
    ancestor(Z, Y).

siblings(X, Y) :-
    parent(P, X),
    parent(P, Y),
    X \== Y.

children(P, Kids) :-
    findall(K, parent(P, K), Kids).

describe(X, Kind) :-
    (   male(X)
    ->  Kind = son(X)
    ;   Kind = daughter(X)
    ).

greeting --> [hello], name.
name --> [world].

main :-
    ancestor(tom, ann),
    format("~w~n", [yes]).
