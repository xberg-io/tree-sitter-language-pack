pragma circom 2.1.6;

include "circomlib/poseidon.circom";

function square(x) {
    return x * x;
}

template Multiplier(n) {
    signal input a;
    signal input b;
    signal output c;
    c <== a * b;
}

template Main() {
    signal input x;
    signal output y;
    component mul = Multiplier(2);
    mul.a <== x;
    mul.b <== square(3);
    y <== mul.c;
}

component main = Main();
