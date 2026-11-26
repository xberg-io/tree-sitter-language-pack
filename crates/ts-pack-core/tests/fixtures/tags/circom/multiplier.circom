pragma circom 2.0.0;

function square(x) {
    return x * x;
}

template Multiplier() {
    signal input a;
    signal input b;
    signal output c;
    c <== a * b;
}

template Square() {
    signal input in;
    signal output out;
    component mult = Multiplier();
    mult.a <== in;
    mult.b <== in;
    out <== mult.c;
    var k = square(2);
}

component main = Square();
