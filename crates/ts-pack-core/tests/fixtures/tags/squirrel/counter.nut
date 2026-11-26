const MAX_COUNT = 10;

enum Color {
    Red,
    Green
}

class Counter {
    count = 0;

    constructor(start) {
        count = start;
    }

    function increment(step) {
        count = clamp(count + step);
        return count;
    }
}

class LoudCounter extends Counter {
    function increment(step) {
        print("incrementing");
        return base.increment(step);
    }
}

function clamp(value) {
    return value > MAX_COUNT ? MAX_COUNT : value;
}

local counter = LoudCounter(0);
counter.increment(2);
