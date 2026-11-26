contract;

use std::hash::Hash;

abi Counter {
    #[storage(read, write)]
    fn increment() -> u64;
}

storage {
    count: u64 = 0,
}

const MAX_COUNT: u64 = 100;

struct Point {
    x: u64,
    y: u64,
}

enum Direction {
    Up: (),
    Down: (),
}

trait Scale {
    fn scale(self, factor: u64) -> Self;
}

impl Scale for Point {
    fn scale(self, factor: u64) -> Self {
        Point { x: self.x * factor, y: self.y * factor }
    }
}

impl Counter for Contract {
    #[storage(read, write)]
    fn increment() -> u64 {
        let next = clamp(storage.count.read() + 1);
        storage.count.write(next);
        next
    }
}

fn clamp(value: u64) -> u64 {
    if value > MAX_COUNT { MAX_COUNT } else { value }
}
