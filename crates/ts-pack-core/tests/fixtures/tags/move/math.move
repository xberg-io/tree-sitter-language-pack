module 0x1::math {
    use std::vector;

    public fun max(a: u64, b: u64): u64 {
        if (a > b) { return a; };
        return b;
    }

    fun square(x: u64): u64 {
        return x * x;
    }

    public fun sum_of_squares(a: u64, b: u64): u64 {
        return square(a) + square(b);
    }
}
