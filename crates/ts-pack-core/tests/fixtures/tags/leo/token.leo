program token.aleo {
    const SUPPLY: u64 = 1000u64;

    struct Metadata {
        name: field,
        decimals: u8,
    }

    record Token {
        owner: address,
        amount: u64,
    }

    mapping balances: address => u64;

    transition mint(receiver: address, amount: u64) -> Token {
        return Token { owner: receiver, amount: amount };
    }

    function add(a: u64, b: u64) -> u64 {
        return a + b;
    }

    inline double(a: u64) -> u64 {
        return add(a, a);
    }
}
