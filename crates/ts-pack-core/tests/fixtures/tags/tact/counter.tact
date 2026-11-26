import "@stdlib/deploy";

const MAX_AMOUNT: Int = 1000;

message Add {
    amount: Int;
}

struct Totals {
    sum: Int;
    count: Int;
}

trait Ownable {
    owner: Address;

    get fun ownerAddress(): Address {
        return self.owner;
    }
}

@name(store_uint)
native storeUint(s: Builder, value: Int, bits: Int): Builder;

asm fun bitCount(x: Int): Int { BITCOUNT }

contract Counter with Deployable, Ownable {
    const STEP: Int = 1;
    owner: Address;
    counter: Int = 0;

    init(owner: Address) {
        self.owner = owner;
    }

    receive(msg: Add) {
        let next: Int = min(self.counter + msg.amount, MAX_AMOUNT);
        self.counter = next;
    }

    receive("increment") {
        self.counter = self.counter + self.STEP;
    }

    get fun totals(): Totals {
        require(sender() == self.ownerAddress(), "not owner");
        return Totals { sum: self.counter, count: bitCount(self.counter) };
    }

    get fun child(): StateInit {
        return initOf Counter(sender());
    }
}

fun double(x: Int): Int {
    return x * 2;
}
