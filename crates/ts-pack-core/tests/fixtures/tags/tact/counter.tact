message Add {
    amount: Int;
}

trait Ownable {
    owner: Address;
}

contract Counter {
    counter: Int = 0;

    receive(msg: Add) {
        self.counter = self.counter + msg.amount;
    }

    get fun value(): Int {
        return self.counter;
    }
}

fun double(x: Int): Int {
    return x * 2;
}
