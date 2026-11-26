---------------------------- MODULE Counter ----------------------------
EXTENDS Naturals

CONSTANT Max

VARIABLE count

Init == count = 0

Next == count < Max /\ count' = count + 1

Double(x) == x * 2

Sum(a, b) == LET s == a + b IN s

inc[n \in Nat] == n + 1

Spec == Init /\ [][Next]_count

THEOREM Safe == Spec => [](count <= Max)
=============================================================================
