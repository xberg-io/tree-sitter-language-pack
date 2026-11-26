import ballerina/io;

const int MAX_SIDES = 8;
final string GREETING = "hello";

type Point record {
    int x;
    int y;
};

enum Color {
    RED,
    GREEN
}

type Shape object {
    function area() returns float;
};

type Sender object {
    remote function send(string message) returns error?;
};

client class Mailer {
    remote function send(string message) returns error? {
        io:println(message);
    }
}

class Counter {
    int count = 0;

    function increment() returns int {
        self.count += 1;
        return self.count;
    }
}

function add(int a, int b) returns int {
    return a + b;
}

public function main() {
    Counter counter = new;
    int total = add(1, 2);
    _ = counter.increment();
    io:println(total);
}
